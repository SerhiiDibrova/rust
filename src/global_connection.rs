package src;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

struct Connection {
    id: String,
    // Additional fields can be added here as needed
}

struct GlobalConnectionContainer {
    connections: Mutex<HashMap<String, Arc<Connection>>>,
}

impl GlobalConnectionContainer {
    fn new() -> Self {
        GlobalConnectionContainer {
            connections: Mutex::new(HashMap::new()),
        }
    }

    fn add_connection(&self, login_id: String, connection: Arc<Connection>) -> Result<(), String> {
        let mut connections = self.connections.lock().map_err(|_| "Failed to lock connections")?;
        if connections.insert(login_id.clone(), connection).is_some() {
            Err(format!("Connection with login_id {} already exists", login_id))
        } else {
            Ok(())
        }
    }

    fn get_connection(&self, login_id: &str) -> Option<Arc<Connection>> {
        let connections = self.connections.lock().ok()?;
        connections.get(login_id).cloned()
    }

    fn remove_connection(&self, login_id: &str) -> Result<(), String> {
        let mut connections = self.connections.lock().map_err(|_| "Failed to lock connections")?;
        if connections.remove(login_id).is_none() {
            Err(format!("No connection found for login_id {}", login_id))
        } else {
            Ok(())
        }
    }
}