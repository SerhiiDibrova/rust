package logging

use log::{info, warn, error, LevelFilter};
use std::fs::File;
use std::io::{Write, Result};
use std::sync::{Mutex, Once};

static INIT: Once = Once::new();
static LOG_FILE: Mutex<Option<File>> = Mutex::new(None);

pub fn init_logging(output_to_file: bool) {
    INIT.call_once(|| {
        if output_to_file {
            let file = File::create("app.log").unwrap();
            *LOG_FILE.lock().unwrap() = Some(file);
        }
        
        let _ = env_logger::builder()
            .filter_level(LevelFilter::Info)
            .format(|buf, record| {
                writeln!(buf, "{} - {}: {}", record.level(), record.target(), record.args())
            })
            .target(env_logger::Target::Stdout)
            .init();
    });
}

pub fn log_info(message: &str) {
    info!("{}", message);
    log_to_file(&format!("INFO: {}", message)).unwrap_or_default();
}

pub fn log_warning(message: &str) {
    warn!("{}", message);
    log_to_file(&format!("WARNING: {}", message)).unwrap_or_default();
}

pub fn log_error(message: &str) {
    error!("{}", message);
    log_to_file(&format!("ERROR: {}", message)).unwrap_or_default();
}

pub fn log_request(request: &str) {
    log_info(&format!("Request: {}", request));
}

pub fn log_response(response: &str) {
    log_info(&format!("Response: {}", response));
}

pub fn log_adaptor_status(adaptor_name: &str, status: &str) {
    log_info(&format!("Adaptor '{}' status: {}", adaptor_name, status));
}

pub fn log_entry_function_status(adaptor_name: &str, found: bool) {
    if found {
        log_info(&format!("Entry function found for adaptor '{}'", adaptor_name));
    } else {
        log_warning(&format!("Entry function not found for adaptor '{}'", adaptor_name));
    }
}

fn log_to_file(message: &str) -> Result<()> {
    let mut file_lock = LOG_FILE.lock().unwrap();
    if let Some(ref mut file) = *file_lock {
        writeln!(file, "{}", message)?;
    }
    Ok(())
}