use std::fs;
use std::io::{self, Read};
use std::path::Path;
use log::{info, error};

pub fn load_adaptor(name: &str, path: &str) -> Result<(), String> {
    info!("Starting to load adaptor: {}", name);
    
    let path = Path::new(path);
    if !path.exists() {
        return Err(format!("Adaptor file not found: {}", path.display()));
    }

    let mut file = fs::File::open(&path).map_err(|e| format!("Failed to open file: {}", e))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).map_err(|e| format!("Failed to read file: {}", e))?;

    if contents.is_empty() {
        error!("Adaptor list is empty");
        return Err("Adaptor list is empty".to_string());
    }

    let required_keys = vec!["key1", "key2", "key3"];
    let mut missing_keys = Vec::new();
    let mut invalid_values = Vec::new();

    for key in &required_keys {
        if !contents.contains(key) {
            missing_keys.push(key);
        } else if contents.contains(&format!("{}: invalid_value", key)) {
            invalid_values.push(key);
        }
    }

    if !missing_keys.is_empty() {
        let error_message = format!("Missing required keys: {:?}", missing_keys);
        error!("{}", error_message);
        return Err(error_message);
    }

    if !invalid_values.is_empty() {
        let error_message = format!("Invalid values for keys: {:?}", invalid_values);
        error!("{}", error_message);
        return Err(error_message);
    }

    info!("Successfully loaded adaptor: {}", name);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use std::path::PathBuf;

    fn create_temp_file(contents: &str) -> PathBuf {
        let mut path = std::env::temp_dir();
        path.push("test_adaptor.txt");
        let mut file = File::create(&path).unwrap();
        file.write_all(contents.as_bytes()).unwrap();
        path
    }

    #[test]
    fn test_load_adaptor_valid() {
        let path = create_temp_file("key1: value\nkey2: value\nkey3: value");
        assert!(load_adaptor("valid_adaptor", path.to_str().unwrap()).is_ok());
    }

    #[test]
    fn test_load_adaptor_missing_file() {
        assert_eq!(load_adaptor("missing_adaptor", "non_existent_file.txt").unwrap_err(), "Adaptor file not found: non_existent_file.txt");
    }

    #[test]
    fn test_load_adaptor_empty_contents() {
        let path = create_temp_file("");
        assert_eq!(load_adaptor("empty_adaptor", path.to_str().unwrap()).unwrap_err(), "Adaptor list is empty");
    }

    #[test]
    fn test_load_adaptor_missing_keys() {
        let path = create_temp_file("key1: value\nkey2: value");
        assert_eq!(load_adaptor("missing_keys_adaptor", path.to_str().unwrap()).unwrap_err(), "Missing required keys: [\"key3\"]");
    }

    #[test]
    fn test_load_adaptor_invalid_values() {
        let path = create_temp_file("key1: invalid_value\nkey2: value\nkey3: value");
        assert_eq!(load_adaptor("invalid_values_adaptor", path.to_str().unwrap()).unwrap_err(), "Invalid values for keys: [\"key1\"]");
    }
}