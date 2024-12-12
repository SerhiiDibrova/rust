use std::fs::{OpenOptions, File};
use std::io::{self, Write};
use std::sync::{Mutex, Once};
use chrono::Local;

static INIT: Once = Once::new();
static mut LOG_FILE: Option<Mutex<File>> = None;

fn init_logger() {
    INIT.call_once(|| {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("app.log")
            .unwrap_or_else(|e| {
                eprintln!("Failed to open log file: {}", e);
                std::process::exit(1);
            });
        unsafe {
            LOG_FILE = Some(Mutex::new(file));
        }
    });
}

pub fn log_info(message: &str) {
    init_logger();
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let log_message = format!("{} [INFO] {}", timestamp, message);
    
    if let Some(ref log_file) = unsafe { &LOG_FILE } {
        if let Ok(mut file) = log_file.lock() {
            if let Err(e) = writeln!(file, "{}", log_message) {
                eprintln!("Failed to write to log file: {}", e);
            }
            if let Err(e) = file.flush() {
                eprintln!("Failed to flush log file: {}", e);
            }
        } else {
            eprintln!("Failed to acquire lock on log file");
        }
    }
    
    if let Err(e) = writeln!(io::stdout(), "{}", log_message) {
        eprintln!("Failed to write to standard output: {}", e);
    }
}