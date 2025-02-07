use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
pub enum AppError {
    ConfigError(String),
    SocketError(String),
    DatabaseError(String),
}

#[derive(Debug)]
pub enum ConnectionError {
    ConnectionFailed(String),
    ReadError(String),
    WriteError(String),
    AlreadyExists(String),
    NotFound(String),
    InvalidInput(String),
}

#[derive(Debug)]
pub enum ServerError {
    InternalError(String),
    NotFoundError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::ConfigError(msg) => write!(f, "ConfigError: {}", msg),
            AppError::SocketError(msg) => write!(f, "SocketError: {}", msg),
            AppError::DatabaseError(msg) => write!(f, "DatabaseError: {}", msg),
        }
    }
}

impl fmt::Display for ConnectionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConnectionError::ConnectionFailed(msg) => write!(f, "ConnectionFailed: {}", msg),
            ConnectionError::ReadError(msg) => write!(f, "ReadError: {}", msg),
            ConnectionError::WriteError(msg) => write!(f, "WriteError: {}", msg),
            ConnectionError::AlreadyExists(msg) => write!(f, "AlreadyExists: {}", msg),
            ConnectionError::NotFound(msg) => write!(f, "NotFound: {}", msg),
            ConnectionError::InvalidInput(msg) => write!(f, "InvalidInput: {}", msg),
        }
    }
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServerError::InternalError(msg) => write!(f, "InternalError: {}", msg),
            ServerError::NotFoundError(msg) => write!(f, "NotFoundError: {}", msg),
        }
    }
}

pub fn handle_error(error: ConnectionError) {
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    log_error(timestamp, &error);
}

fn log_error(timestamp: u64, error: &dyn fmt::Display) {
    println!("[{}] ERROR: {}", timestamp, error);
}