use std::{path::Path, fs, time::{SystemTime, UNIX_EPOCH, SystemTimeError}};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub path: Option<String>, 
    pub last_updated: u64,
} 

impl Config {
    pub fn new() -> Self {
        Self {
            path: None, 
            last_updated: 0
        }
    } 

    pub fn load(&mut self, path: &str) -> Result<Config, ConfigError> {
        // Check if file exists
        if !Path::new(path).exists() {
            return Err(ConfigError::InvalidPath)
        } 

        let file = fs::File::open(path)
            .map_err(ConfigError::IoError)?;

        let config: Config = serde_yaml::from_reader(file)
            .map_err(ConfigError::YamlError)?;

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(ConfigError::SystemTimeError)?
            .as_secs();

        let new_config = Config {
            path: config.path,
            last_updated: now,
        };

        Ok(new_config)
    }
} 

#[derive(Debug)]
pub enum ConfigError {
    InvalidPath, 
    IoError(std::io::Error),
    YamlError(serde_yaml::Error),
    SystemTimeError(std::time::SystemTimeError),
} 

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidPath => write!(f, "Invalid path provided"),
            Self::IoError(err) => write!(f, "I/O error: {}", err),
            Self::YamlError(err) => write!(f, "Error parsing YAML: {}", err),
            Self::SystemTimeError(err) => write!(f, "Error getting System time: {}", err),
        } 
    }
} 

impl std::error::Error for ConfigError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::IoError(err) => Some(err),
            Self::YamlError(err) => Some(err),
            Self::SystemTimeError(err) => Some(err),
            _ => None,
        } 
    }
} 

impl From<std::io::Error> for ConfigError {
    fn from(err: std::io::Error) -> Self {
        ConfigError::IoError(err)
    } 
} 

impl From<SystemTimeError> for ConfigError {
    fn from(err: SystemTimeError) -> Self {
        ConfigError::SystemTimeError(err)
    }
} 


#[cfg(test)]
mod tests {

    use std::{fs::{File, Permissions}, env, os::unix::fs::PermissionsExt};

    use chrono::{Duration, Utc, DateTime};

    // use config_test_fixtures::{MockSystemTime, SystemTime, MyCustomMockError};
    use crate::tests_common::test_setup;
    

    use super::*;

    #[test]
    fn test_new_config() {
        let config = Config::new();

        // Assert default fields 
        assert_eq!(config.path, None);
        assert_eq!(config.last_updated, 0);
    } 

    #[test]
    fn test_load_config() {
        let mut config = Config::new();

        // Load invalid path
        let result = config.load("invalid.yml");

        // Assert error 
        assert!(result.is_err());

        // Generate valid yaml string 
        let yaml = test_setup::create_test_config();

        // Write string to temp file 
        let tmp_file_path = "tmp.yml";
        std::fs::write(tmp_file_path, yaml);

        // Assert file exists 
        assert!(Path::new(tmp_file_path).exists());

        // Load Valid Path 
        let result = config.load("tmp.yml");

        // Assert Success
        assert!(result.is_ok());

        if let Ok(c) = result {
            config = c;
        }; 


        // Assert fields populated 
        assert_eq!(config.path, Some("tmp.yml".into()));
        assert!(config.last_updated > 0);

        std::fs::remove_file(tmp_file_path);
    } 

    #[test]
    fn test_load_invalid_path() {
        let mut config = Config::new();

        let result = config.load("invalid.yaml");

        assert!(matches!(result, Err(ConfigError::InvalidPath)));
    } 

    
    #[test]
    fn test_load_io_error() {
        let mut config = Config::new();
        let tmp_dir = env::temp_dir();
        let test_path = tmp_dir.with_file_name("test_load_io_error.yaml");

        let binding = test_path.clone();
        let test_path_return = binding.to_str().unwrap();

        // open file
        File::create(test_path.clone());

        // Deny all permissions
        let mut perms = fs::metadata(test_path.clone())
            .expect("Error getting metadata")
            .permissions();

        // Set readonly 
        perms.set_readonly(true);

        // Temporarily set permissions
        std::fs::set_permissions(test_path.clone(), perms)
            .expect("Failed to change permissions");

        let result = config.load(&test_path_return);
        println!("error = {:?}", result.err());
        // assert!(matches!(result, Err(ConfigError::IoError(_))));

        // Restor original permissions
        let mut perms = fs::metadata(test_path.clone())
            .expect("Error getting metadata")
            .permissions();

        perms.set_readonly(false);

        fs::set_permissions(test_path.clone(), perms)
            .expect("Failed to restore permissions");
    } 

    #[test]
    fn test_load_yaml_error() {
        let mut config = Config::new();

        let yaml = "invalid";

        let tmp_file = "tmp.yaml";
        std::fs::write(tmp_file, yaml);

        let result = config.load(tmp_file);

        assert!(matches!(result, Err(ConfigError::YamlError(_))));
    } 

    #[test]
    fn test_load_system_time_error() {
        // Mock systemtime to return error 
        let now = DateTime::<Utc>::from(SystemTime::now());
        let offset = chrono::Duration::days(1);
        let future_time = now + offset;

        let future_sys_time: SystemTime = future_time.into();

        let result = SystemTime::now().duration_since(future_sys_time);
        // let result = future_sys_time.duration_since(SystemTime::now());

        let system_time_err = result.unwrap_err();

        let config_err = ConfigError::from(system_time_err);

        assert!(matches!(config_err, ConfigError::SystemTimeError(_)));
    } 
} 
