use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use serde_json::Value;

pub struct Storage {
    packets: Arc<Mutex<HashMap<String, String>>>,
}

impl Storage {
    pub fn new() -> Self {
        Storage {
            packets: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn insert_packet(&self, key: String, json_string: String) {
        let mut packets = self.packets.lock().unwrap();
        packets.insert(key, json_string);
    }

    pub fn get_packet(&self, key: &str) -> Option<String> {
        let packets = self.packets.lock().unwrap();
        packets.get(key).cloned()
    }
}