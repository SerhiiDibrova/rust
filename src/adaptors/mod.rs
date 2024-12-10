package adaptors;

use std::collections::HashMap;
use std::error::Error;
use std::fs;
use serde::Deserialize;
use regex::Regex;

#[derive(Deserialize)]
pub struct AdaptorConfig {
    pub name: String,
    pub version: String,
}

pub struct Adaptor {
    pub name: String,
    pub version: String,
}

pub struct AdaptorManager {
    adaptors: HashMap<String, Adaptor>,
}

impl AdaptorManager {
    pub fn new() -> Self {
        AdaptorManager {
            adaptors: HashMap::new(),
        }
    }

    pub fn load_adaptors_from_config(&mut self, config_path: &str) -> Result<(), Box<dyn Error>> {
        let config_data = fs::read_to_string(config_path)?;
        let adaptors: Vec<AdaptorConfig> = serde_json::from_str(&config_data).map_err(|e| format!("JSON parsing error: {}", e))?;

        for adaptor_config in adaptors {
            self.load_adaptor(&adaptor_config.name, &adaptor_config.version)?;
        }
        Ok(())
    }

    pub fn load_adaptor(&mut self, name: &str, version: &str) -> Result<(), Box<dyn Error>> {
        if self.adaptors.contains_key(name) {
            return Err(format!("Adaptor with name '{}' already exists.", name).into());
        }
        if !self.validate_version_format(version) {
            return Err(format!("Invalid version format for adaptor '{}': {}", name, version).into());
        }
        let adaptor = Adaptor {
            name: name.to_string(),
            version: version.to_string(),
        };
        self.adaptors.insert(name.to_string(), adaptor);
        Ok(())
    }

    pub fn get_adaptor(&self, name: &str) -> Option<&Adaptor> {
        self.adaptors.get(name)
    }

    fn validate_version_format(&self, version: &str) -> bool {
        let re = Regex::new(r"^\d+\.\d+(\.\d+)?$").unwrap();
        re.is_match(version)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_adaptor() {
        let mut manager = AdaptorManager::new();
        assert!(manager.load_adaptor("TestAdaptor", "1.0").is_ok());
        assert!(manager.get_adaptor("TestAdaptor").is_some());
    }

    #[test]
    fn test_load_adaptor_duplicate() {
        let mut manager = AdaptorManager::new();
        manager.load_adaptor("TestAdaptor", "1.0").unwrap();
        assert!(manager.load_adaptor("TestAdaptor", "1.0").is_err());
    }

    #[test]
    fn test_load_adaptor_invalid_version() {
        let mut manager = AdaptorManager::new();
        assert!(manager.load_adaptor("TestAdaptor", "invalid_version").is_err());
    }

    #[test]
    fn test_load_adaptors_from_config() {
        let mut manager = AdaptorManager::new();
        let config = r#"[{"name": "Adaptor1", "version": "1.0"}, {"name": "Adaptor2", "version": "2.0"}]"#;
        let config_path = "test_config.json";
        fs::write(config_path, config).unwrap();
        assert!(manager.load_adaptors_from_config(config_path).is_ok());
        assert!(manager.get_adaptor("Adaptor1").is_some());
        assert!(manager.get_adaptor("Adaptor2").is_some());
        fs::remove_file(config_path).unwrap();
    }

    #[test]
    fn test_load_adaptors_from_config_invalid_json() {
        let mut manager = AdaptorManager::new();
        let invalid_config_path = "invalid_config.json";
        fs::write(invalid_config_path, "invalid json").unwrap();
        assert!(manager.load_adaptors_from_config(invalid_config_path).is_err());
        fs::remove_file(invalid_config_path).unwrap();
    }
}