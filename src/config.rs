pub struct Config {

} 

impl Config {
} 

#[test]
mod tests {
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

        // Load Valid Path 
        let result = config.load("tests/data/config.yml");

        // Assert Success
        assert!(result.is_ok());


        // Assert fields populated 
        assert_eq!(config.path, Some("tests/data/config.yml".into()));
        assert!(config.last_updated > 0);
    } 
} 
