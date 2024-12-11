package utils;

use log::{info, error, warn, debug, trace};
use std::sync::Once;

static INIT: Once = Once::new();

pub fn init_logger() {
    INIT.call_once(|| {
        let _ = env_logger::builder().format_timestamp(None).init();
    });
}

pub fn log_info(message: &str) {
    info!("{}", message);
}

pub fn log_error(message: &str) {
    error!("{}", message);
}

pub fn log_warn(message: &str) {
    warn!("{}", message);
}

pub fn log_debug(message: &str) {
    debug!("{}", message);
}

pub fn log_trace(message: &str) {
    trace!("{}", message);
}

pub fn log_action(action: &str) {
    info!("Action performed: {}", action);
}

pub fn log_significant_event(event: &str) {
    info!("Significant event: {}", event);
}