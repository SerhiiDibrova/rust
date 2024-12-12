use std::sync::{Once, Mutex};
use log::{info, error, LevelFilter};
use simplelog::{Config, SimpleLogger};

pub struct Logger {
    mutex: Mutex<()>,
}

static INIT: Once = Once::new();
static mut LOGGER: Option<Logger> = None;

impl Logger {
    pub fn init() -> Result<(), Box<dyn std::error::Error>> {
        INIT.call_once(|| {
            SimpleLogger::init(LevelFilter::Info, Config::default()).unwrap();
            unsafe {
                LOGGER = Some(Logger {
                    mutex: Mutex::new(),
                });
            }
        });
        Ok(())
    }

    pub fn log_info(&self, message: &str) {
        let _guard = self.mutex.lock().unwrap();
        info!("{}", message);
    }

    pub fn log_error(&self, message: &str) {
        let _guard = self.mutex.lock().unwrap();
        error!("{}", message);
    }
}

pub fn get_logger() -> &'static Logger {
    Logger::init().expect("Logger initialization failed");
    unsafe { LOGGER.as_ref().unwrap() }
}

pub fn log_info(message: &str) {
    get_logger().log_info(message);
}

pub fn log_error(message: &str) {
    get_logger().log_error(message);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_logging() {
        let messages = Arc::new(Mutex::new(vec![]));
        let messages_clone = Arc::clone(&messages);

        let handles: Vec<_> = (0..10).map(|i| {
            let messages_clone = Arc::clone(&messages_clone);
            thread::spawn(move || {
                log_info(&format!("Info message {}", i));
                log_error(&format!("Error message {}", i));
                messages_clone.lock().unwrap().push(i);
            })
        }).collect();

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(messages.lock().unwrap().len(), 10);
    }
}