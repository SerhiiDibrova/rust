use std::fs::{File, read_to_string};
use std::io::Error as IoError;
use serde_json::Value;
use log::{error, info};

pub fn load_config(path: &str) -> Result<(), String> {
    let file = File::open(path).map_err(|e| {
        error!("Failed to open config file: {}", e);
        format!("Failed to open config file: {}", e)
    })?;

    let content = read_to_string(file).map_err(|e| {
        error!("Failed to read config file: {}", e);
        format!("Failed to read config file: {}", e)
    })?;

    let json: Value = serde_json::from_str(&content).map_err(|e| {
        error!("Failed to parse JSON: {}", e);
        format!("Failed to parse JSON: {}", e)
    })?;

    let server = json.get("server").ok_or_else(|| {
        let msg = "Missing 'server' key in config".to_string();
        error!("{}", msg);
        msg
    })?;

    let port = server.get("port").and_then(Value::as_u64).ok_or_else(|| {
        let msg = "Invalid or missing 'port' in 'server'".to_string();
        error!("{}", msg);
        msg
    })?;

    if port > 65535 {
        let msg = "Port number must be between 0 and 65535".to_string();
        error!("{}", msg);
        return Err(msg);
    }

    info!("Starting server on port {}", port);

    let database = json.get("database").and_then(Value::as_str).ok_or_else(|| {
        let msg = "Invalid or missing 'database' key".to_string();
        error!("{}", msg);
        msg
    })?;

    if database.is_empty() {
        let msg = "Database string cannot be empty".to_string();
        error!("{}", msg);
        return Err(msg);
    }

    // Initialize database here

    let adaptors = json.get("adaptors").ok_or_else(|| {
        let msg = "Missing 'adaptors' key".to_string();
        error!("{}", msg);
        msg
    })?;

    let adaptors_array = adaptors.as_array().ok_or_else(|| {
        let msg = "'adaptors' key must be an array".to_string();
        error!("{}", msg);
        msg
    })?;

    for adaptor in adaptors_array {
        let exchange = adaptor.get("exchange").and_then(Value::as_str).ok_or_else(|| {
            let msg = "Missing 'exchange' in adaptor".to_string();
            error!("{}", msg);
            msg
        })?;

        if exchange.is_empty() {
            let msg = "Exchange string cannot be empty".to_string();
            error!("{}", msg);
            return Err(msg);
        }

        let path = adaptor.get("path").and_then(Value::as_str).ok_or_else(|| {
            let msg = "Missing 'path' in adaptor".to_string();
            error!("{}", msg);
            msg
        })?;

        if path.is_empty() {
            let msg = "Path string cannot be empty".to_string();
            error!("{}", msg);
            return Err(msg);
        }

        info!("Loading adaptor: {} from path: {}", exchange, path);
        // Load adaptor here
    }

    Ok(())
}