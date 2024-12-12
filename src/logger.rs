use log::{error, warn, LevelFilter};
use simplelog::{Config, SimpleLogger};
use std::time::SystemTime;

pub fn init_logging() {
    SimpleLogger::init(LevelFilter::Info, Config::default()).unwrap();
}

pub fn log_warning(function_name: &str, message: &str, error_code: &str) {
    warn!("[WARNING] {}: {} (Error Code: {})", function_name, message, error_code);
}

pub fn log_error(function_name: &str, message: &str) {
    let timestamp = SystemTime::now();
    let time = timestamp.duration_since(SystemTime::UNIX_EPOCH).unwrap_or_default();
    if let Err(e) = error!("[ERROR] [{}] {}: {}", time.as_secs(), function_name, message) {
        eprintln!("Logging error: {}", e);
    }
}

pub fn log_success(function_name: &str, message: &str) {
    println!("[SUCCESS] {}: {}", function_name, message);
}