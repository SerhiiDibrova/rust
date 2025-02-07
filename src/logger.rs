use log::{info, error, warn, debug, LevelFilter};
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::Mutex;
use chrono::Local;

lazy_static::lazy_static! {
    static ref LOG_FILE: Mutex<Option<std::fs::File>> = Mutex::new(None);
}

pub fn init_logger() {
    let _ = env_logger::builder()
        .filter_level(LevelFilter::Info)
        .format(|buf, record| {
            let now = Local::now();
            writeln!(buf, "{} [{}] - {}", now.format("%Y-%m-%d %H:%M:%S"), record.level(), record.args())
        })
        .target(env_logger::Target::Stdout)
        .init();
}

pub fn log_info(message: &str) {
    info!("{}", message);
}

pub fn log_error(error_details: &str) {
    error!("{}", error_details);
}

pub fn log_warning(message: &str) {
    warn!("{}", message);
}

pub fn log_debug(message: &str) {
    debug!("{}", message);
}

pub fn log_event(message: &str) {
    let now = Local::now();
    info!("{} - {}", now.format("%Y-%m-%d %H:%M:%S"), message);
}