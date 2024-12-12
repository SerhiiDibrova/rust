use std::fs::File;
use std::io::Read;
use std::path::Path;
use serde::Deserialize;
use log::{error};
use std::process;

#[derive(Deserialize)]
pub struct Config {
    pub server: String,
    pub database: String,
    pub adaptors: Vec<String>,
}

pub fn load_config(file_path: &str) -> Result<Config, String> {
    let path = Path::new(file_path);
    if !path.exists() {
        let error_message = format!("Configuration file does not exist: {}", file_path);
        error!("{}", error_message);
        error!("Aborting application due to inability to load configuration.");
        process::exit(1);
    }

    let mut file = File::open(file_path).map_err(|e| {
        let error_message = format!("Failed to open configuration file: {}. Error: {}", file_path, e);
        error!("{}", error_message);
        error!("Aborting application due to inability to load configuration.");
        process::exit(1);
    })?;
    
    let mut contents = String::new();
    file.read_to_string(&mut contents).map_err(|e| {
        let error_message = format!("Failed to read configuration file: {}. Error: {}", file_path, e);
        error!("{}", error_message);
        error!("Aborting application due to inability to load configuration.");
        process::exit(1);
    })?;
    
    if contents.is_empty() {
        let error_message = format!("Configuration file is empty: {}", file_path);
        error!("{}", error_message);
        error!("Aborting application due to inability to load configuration.");
        process::exit(1);
    }
    
    let config: Config = serde_json::from_str(&contents).map_err(|e| {
        let error_message = format!("Failed to parse configuration file: {}. Error: {}", file_path, e);
        error!("{}", error_message);
        error!("Aborting application due to inability to load configuration.");
        process::exit(1);
    })?;
    
    let mut missing_keys = Vec::new();
    if config.server.is_empty() {
        missing_keys.push("server");
    }
    if config.database.is_empty() {
        missing_keys.push("database");
    }
    if config.adaptors.is_empty() {
        missing_keys.push("adaptors");
    }
    
    if !missing_keys.is_empty() {
        let error_message = format!("Missing required configuration keys: {:?}", missing_keys);
        error!("{}", error_message);
        process::exit(1);
    }
    
    Ok(config)
}