use std::collections::HashMap;

pub struct KeyValueStore {
    store: HashMap<String, String>,
}

impl KeyValueStore {
    pub fn new() -> Self {
        KeyValueStore {
            store: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: String) {
        self.store.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.store.get(key)
    }

    pub fn remove(&mut self, key: &str) -> Option<String> {
        self.store.remove(key)
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.store.contains_key(key)
    }

    pub fn keys(&self) -> Vec<String> {
        self.store.keys().cloned().collect()
    }

    pub fn values(&self) -> Vec<String> {
        self.store.values().cloned().collect()
    }

    pub fn clear(&mut self) {
        self.store.clear();
    }
}