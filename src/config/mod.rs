package config

use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub server_port: u16,
    pub database_url: String,
    pub log_level: String,
    pub api_key: String,
    pub timeout: u64,
}

pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let mut file = File::open("merlin.json").map_err(|e| format!("File not found: {}", e))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).map_err(|e| format!("Error reading file: {}", e))?;
    let config: Config = serde_json::from_str(&contents).map_err(|e| format!("Error parsing JSON: {}", e))?;
    Ok(config)
}