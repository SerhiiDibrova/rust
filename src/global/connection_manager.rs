mod connection_manager {
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};
    use std::time::{Duration, Instant};

    #[derive(Debug, Clone)]
    pub struct Connection {
        id: String,
        last_active: Instant,
    }

    pub struct ConnectionManager {
        connections: Arc<Mutex<HashMap<String, Connection>>>,
        timeout: Duration,
    }

    impl ConnectionManager {
        pub fn new(timeout: Duration) -> Self {
            ConnectionManager {
                connections: Arc::new(Mutex::new(HashMap::new())),
                timeout,
            }
        }

        pub fn add_connection(&self, id: String) -> Result<(), String> {
            if !Self::is_valid_id(&id) {
                return Err("Invalid connection ID".to_string());
            }
            let mut connections = self.connections.lock().unwrap();
            if connections.contains_key(&id) {
                return Err("Connection ID already exists".to_string());
            }
            connections.insert(id.clone(), Connection { id, last_active: Instant::now() });
            Ok(())
        }

        pub fn remove_connection(&self, id: &str) -> Result<(), String> {
            let mut connections = self.connections.lock().unwrap();
            if connections.remove(id).is_none() {
                return Err("Connection ID does not exist".to_string());
            }
            Ok(())
        }

        pub fn connection_exists(&self, id: &str) -> bool {
            let connections = self.connections.lock().unwrap();
            connections.contains_key(id)
        }

        pub fn get_connection(&self, id: &str) -> Result<Connection, String> {
            let connections = self.connections.lock().unwrap();
            connections.get(id).cloned().ok_or_else(|| "Connection ID does not exist".to_string())
        }

        pub fn cleanup_inactive_connections(&self) {
            let mut connections = self.connections.lock().unwrap();
            let now = Instant::now();
            connections.retain(|_, conn| now.duration_since(conn.last_active) < self.timeout);
        }

        pub fn validate_connection_id(id: &str) -> bool {
            Self::is_valid_id(id)
        }

        fn is_valid_id(id: &str) -> bool {
            !id.is_empty() && id.chars().all(char::is_alphanumeric)
        }
    }
}