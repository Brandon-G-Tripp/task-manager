use std::{path::Path, fs, time::{SystemTime, UNIX_EPOCH}};
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

    pub fn load(&mut self, path: &str) -> Result<(), ConfigError> {
        // Check if file exists
        if !Path::new(path).exists() {
            return Err(ConfigError::InvalidPath)
        } 

        Ok(())
    }
} 

#[derive(Debug)]
pub enum ConfigError {
    InvalidPath, 
    IoError(std::io::Error),
    YamlError(serde_yaml::Error)
} 

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidPath => write!(f, "Invalid path provided"),
            Self::IoError(err) => write!(f, "I/O error: {}", err),
            Self::YamlError(err) => write!(f, "Error parsing YAML: {}", err),
        } 
    }
} 

impl std::error::Error for ConfigError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::IoError(err) => Some(err),
            Self::YamlError(err) => Some(err),
            _ => None,
        } 
    }
} 

#[cfg(test)]
mod tests {
    use crate::test_setup::create_test_config;

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
        let yaml = create_test_config();

        // Write string to temp file 
        let tmp_file = "tmp.yml";
        std::fs::write(tmp_file, yaml);

        // Load Valid Path 
        let mut result = config.load("tests/data/config.yml");

        // Assert Success
        assert!(result.is_ok());


        // Assert fields populated 
        assert_eq!(config.path, Some("tests/data/config.yml".into()));
        assert!(config.last_updated > 0);

        std::fs::remove_file(tmp_file);
    } 
} 
