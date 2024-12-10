package global_connection_manager

use std::collections::HashMap;
use std::sync::Mutex;
use log::{error};

struct Connection;

struct GlobalConnectionManager {
    _globalConnectionPtrContainer: Mutex<HashMap<String, *const Connection>>,
}

impl GlobalConnectionManager {
    fn new() -> Self {
        GlobalConnectionManager {
            _globalConnectionPtrContainer: Mutex::new(HashMap::new()),
        }
    }

    fn new_connection_requested(&self, login_id: String, connection: *const Connection) {
        let mut container = self._globalConnectionPtrContainer.lock().unwrap();
        container.insert(login_id, connection);
    }

    fn get_connection(&self, login_id: &str) -> Option<*const Connection> {
        let container = self._globalConnectionPtrContainer.lock().unwrap();
        container.get(login_id).copied()
    }

    fn connection_closed(&self, login_id: &str) {
        let mut container = self._globalConnectionPtrContainer.lock().unwrap();
        if container.remove(login_id).is_none() {
            error!("Failed to close connection: {} does not exist", login_id);
        }
    }
}