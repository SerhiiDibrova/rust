package global_event_container

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

struct GlobalEventContainer {
    container: Arc<Mutex<HashMap<String, String>>>,
}

impl GlobalEventContainer {
    fn new() -> Self {
        GlobalEventContainer {
            container: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn add_strategy(&self, token: String, strategy: String) -> Result<(), String> {
        let mut map = self.container.lock().map_err(|_| "Failed to acquire lock".to_string())?;
        if map.contains_key(&token) {
            return Err("Token already exists".to_string());
        }
        map.insert(token, strategy);
        Ok(())
    }

    fn retrieve_strategy(&self, token: &str) -> Result<Option<String>, String> {
        let map = self.container.lock().map_err(|_| "Failed to acquire lock".to_string())?;
        Ok(map.get(token).cloned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_add_and_get_entry() {
        let container = GlobalEventContainer::new();
        assert!(container.add_strategy("token1".to_string(), "strategy1".to_string()).is_ok());
        assert_eq!(container.retrieve_strategy("token1").unwrap(), Some("strategy1".to_string()));
    }

    #[test]
    fn test_add_duplicate_entry() {
        let container = GlobalEventContainer::new();
        assert!(container.add_strategy("token1".to_string(), "strategy1".to_string()).is_ok());
        assert_eq!(container.add_strategy("token1".to_string(), "strategy2".to_string()), Err("Token already exists".to_string()));
    }

    #[test]
    fn test_get_nonexistent_entry() {
        let container = GlobalEventContainer::new();
        assert_eq!(container.retrieve_strategy("nonexistent").unwrap(), None);
    }

    #[test]
    fn test_concurrent_access() {
        let container = Arc::new(GlobalEventContainer::new());
        let mut handles = vec![];

        for i in 0..10 {
            let container_clone = Arc::clone(&container);
            handles.push(thread::spawn(move || {
                container_clone.add_strategy(format!("token{}", i), format!("strategy{}", i)).unwrap();
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        for i in 0..10 {
            assert_eq!(container.retrieve_strategy(&format!("token{}", i)).unwrap(), Some(format!("strategy{}", i)));
        }
    }
}