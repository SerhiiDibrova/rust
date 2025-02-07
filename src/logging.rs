use std::fs::OpenOptions;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn log_connection_removal(login_id: &str) {
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let log_entry = format!("{}: Connection removed for loginId: {}\n", timestamp, login_id);
    let mut file = OpenOptions::new().append(true).create(true).open("connection_log.txt").unwrap();
    file.write_all(log_entry.as_bytes()).unwrap();
}

pub fn log_connection_not_found(login_id: &str) {
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let log_entry = format!("{}: Attempted to remove non-existent connection for loginId: {}\n", timestamp, login_id);
    let mut file = OpenOptions::new().append(true).create(true).open("connection_log.txt").unwrap();
    file.write_all(log_entry.as_bytes()).unwrap();
}