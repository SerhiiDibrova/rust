package logging

use std::fs::{File, OpenOptions};
use std::io::{Write, Error};
use std::sync::{Mutex, Once};
use log::{LevelFilter, Log, Metadata, Record, SetLoggerError};

struct SimpleLogger {
    level: LevelFilter,
    log_to_file: bool,
    file: Option<File>,
}

impl Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            if self.log_to_file {
                if let Err(e) = self.write_to_file(record) {
                    eprintln!("Failed to write to log file: {}", e);
                }
            } else {
                println!("{} - {}", record.level(), record.args());
            }
        }
    }

    fn flush(&self) {
        if let Some(file) = &self.file {
            let _ = file.flush();
        }
    }

    fn write_to_file(&mut self, record: &Record) -> Result<(), Error> {
        if self.file.is_none() {
            let file = OpenOptions::new()
                .create(true)
                .append(true)
                .open("log.txt")?;
            self.file = Some(file);
        }
        if let Some(file) = &mut self.file {
            writeln!(file, "{} - {}", record.level(), record.args())?;
        }
        Ok(())
    }
}

static LOGGER: Mutex<Option<SimpleLogger>> = Mutex::new(None);
static INIT: Once = Once::new();

pub fn init_logging(log_to_file: bool) -> Result<(), SetLoggerError> {
    INIT.call_once(|| {
        let logger = SimpleLogger {
            level: LevelFilter::Info,
            log_to_file,
            file: None,
        };
        let mut guard = LOGGER.lock().unwrap();
        *guard = Some(logger);
        log::set_boxed_logger(Box::new(logger)).unwrap();
        log::set_max_level(LevelFilter::Info);
    });
    Ok(())
}