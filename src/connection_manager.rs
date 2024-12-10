package src;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::logger;

pub struct Connection {
    pub address: String,
    pub port: u16,
}

pub struct ConnectionManager {
    connections: Arc<Mutex<HashMap<u32, Connection>>>,
}

impl ConnectionManager {
    pub fn new() -> Self {
        ConnectionManager {
            connections: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn add_connection(&self, login_id: u32, connection: Connection) {
        let mut connections = self.connections.lock().unwrap();
        connections.insert(login_id, connection);
    }

    pub fn connection_closed(&self, login_id: u32) {
        let mut connections = self.connections.lock().unwrap();
        if connections.remove(&login_id).is_some() {
            logger::log_connection_closed(login_id);
        }
    }

    pub fn get_connection(&self, login_id: u32) -> Option<Connection> {
        let connections = self.connections.lock().unwrap();
        connections.get(&login_id).cloned()
    }

    pub fn has_connection(&self, login_id: u32) -> bool {
        let connections = self.connections.lock().unwrap();
        connections.contains_key(&login_id)
    }
}