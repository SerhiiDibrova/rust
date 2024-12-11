package logger

use std::sync::{Mutex, Once};
use log::{info, error, LevelFilter};
use std::time::{SystemTime, UNIX_EPOCH};

struct Logger {
    initialized: Once,
    mutex: Mutex<()>,
}

impl Logger {
    fn new() -> Self {
        Logger {
            initialized: Once::new(),
            mutex: Mutex::new(),
        }
    }

    fn init(&self) {
        self.initialized.call_once(|| {
            let _guard = self.mutex.lock().unwrap();
            log::set_max_level(LevelFilter::Info);
        });
    }

    fn log(&self, level: log::Level, message: &str, context: &str) {
        let _guard = self.mutex.lock().unwrap();
        let timestamp = Self::current_timestamp();
        match level {
            log::Level::Error => error!("{} - {} - {}", timestamp, context, message),
            log::Level::Info => info!("{} - {} - {}", timestamp, context, message),
            _ => {}
        }
    }

    fn current_timestamp() -> String {
        let start = SystemTime::now();
        let duration = start.duration_since(UNIX_EPOCH).unwrap();
        format!("{:?}", duration.as_secs())
    }
}

static LOGGER: Logger = Logger::new();

pub fn init_logger() {
    LOGGER.init();
}

pub fn log_info(message: &str, context: &str) {
    LOGGER.log(log::Level::Info, message, context);
}

pub fn log_error(message: &str, context: &str) {
    LOGGER.log(log::Level::Error, message, context);
}