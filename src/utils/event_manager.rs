package utils;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use log::{info};

struct EventManager {
    event_container: Arc<Mutex<HashMap<String, String>>>,
}

impl EventManager {
    fn new() -> Self {
        EventManager {
            event_container: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn register_for_data(&self, token: String, strategy_address: String) -> Result<(), String> {
        if token.is_empty() {
            return Err("Token cannot be empty".to_string());
        }

        let mut container = self.event_container.lock().map_err(|_| "Failed to acquire lock".to_string())?;
        container.insert(token.clone(), strategy_address.clone());
        info!("Registered for data updates with token: {}", token);
        Ok(())
    }
}