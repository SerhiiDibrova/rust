package config

use serde::{Deserialize, Serialize};
use std::fs;
use std::error::Error;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub server_port: u16,
    pub database_connection: String,
    pub adaptors: Vec<String>,
}

impl Config {
    pub fn load_from_file(file_path: &str) -> Result<Self, Box<dyn Error>> {
        let data = fs::read_to_string(file_path).map_err(|e| format!("Failed to read file: {}", e))?;
        let config: Config = serde_json::from_str(&data).map_err(|e| format!("Failed to parse JSON: {}", e))?;
        if config.server_port == 0 {
            return Err("server_port must be greater than 0".into());
        }
        if config.database_connection.is_empty() {
            return Err("database_connection cannot be empty".into());
        }
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;

    fn create_test_config_file() {
        let json_data = r#"
        {
            "server_port": 8080,
            "database_connection": "postgres://user:password@localhost/db",
            "adaptors": ["json", "xml"]
        }
        "#;
        let mut file = File::create("config.json").unwrap();
        file.write_all(json_data.as_bytes()).unwrap();
    }

    fn create_invalid_test_config_file() {
        let json_data = r#"
        {
            "server_port": 0,
            "database_connection": "",
            "adaptors": ["json", "xml"]
        }
        "#;
        let mut file = File::create("invalid_config.json").unwrap();
        file.write_all(json_data.as_bytes()).unwrap();
    }

    #[test]
    fn test_load_from_file() {
        create_test_config_file();
        let config = Config::load_from_file("config.json");
        assert!(config.is_ok());
        if let Ok(cfg) = config {
            assert_eq!(cfg.server_port, 8080);
            assert_eq!(cfg.database_connection, "postgres://user:password@localhost/db");
            assert_eq!(cfg.adaptors, vec!["json", "xml"]);
        }
    }

    #[test]
    fn test_load_from_file_invalid_json() {
        create_invalid_test_config_file();
        let config = Config::load_from_file("invalid_config.json");
        assert!(config.is_err());
    }

    #[test]
    fn test_load_from_file_not_found() {
        let config = Config::load_from_file("non_existent_file.json");
        assert!(config.is_err());
    }
}