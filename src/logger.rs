package logger;

use log::{LevelFilter, Log, Metadata, Record};
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

struct Logger {
    file: Arc<Mutex<std::fs::File>>,
}

impl Logger {
    pub fn new(file_path: &str) -> Logger {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(file_path)
            .unwrap();
        Logger {
            file: Arc::new(Mutex::new(file)),
        }
    }

    pub fn log_message(&self, message: &str) {
        self.log_event(message);
    }

    pub fn log_event(&self, message: &str) {
        self.write_log("EVENT", message);
    }

    pub fn log_error(&self, message: &str) {
        self.write_log("ERROR", message);
    }

    pub fn log_info(&self, message: &str) {
        self.write_log("INFO", message);
    }

    pub fn log_warning(&self, message: &str) {
        self.write_log("WARNING", message);
    }

    fn write_log(&self, level: &str, message: &str) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let log_entry = format!("{} [{}] {}\n", timestamp, level, message);
        let mut file = self.file.lock().unwrap();
        file.write_all(log_entry.as_bytes()).unwrap();
    }
}

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= log::max_level()
    }

    fn log(&self, record: &Record) {
        let message = record.args().to_string();
        match record.level() {
            log::Level::Error => self.log_error(&message),
            log::Level::Warn => self.log_warning(&message),
            log::Level::Info => self.log_info(&message),
            log::Level::Debug | log::Level::Trace => self.log_event(&message),
        }
    }

    fn flush(&self) {}
}

pub fn init_logger(file_path: &str) {
    let logger = Logger::new(file_path);
    log::set_boxed_logger(Box::new(logger)).map(|()| log::set_max_level(LevelFilter::Warn)).unwrap();
}