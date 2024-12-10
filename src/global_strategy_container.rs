package src;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct Strategy {
    pub parameter1: String,
    pub parameter2: i32,
}

pub struct GlobalStrategyContainer {
    strategies: Arc<Mutex<HashMap<u32, Strategy>>>,
}

impl GlobalStrategyContainer {
    pub fn new() -> Self {
        GlobalStrategyContainer {
            strategies: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn get_strategy(&self, id: u32) -> Option<Strategy> {
        let strategies = self.strategies.lock().unwrap();
        strategies.get(&id).cloned()
    }

    pub fn update_strategy(&self, id: u32, strategy: Strategy) -> Result<(), String> {
        let mut strategies = self.strategies.lock().map_err(|_| "Failed to lock mutex".to_string())?;
        if strategies.contains_key(&id) {
            strategies.insert(id, strategy);
            Ok(())
        } else {
            Err("Strategy not found".to_string())
        }
    }

    pub fn remove_strategy(&self, id: u32) -> Result<(), String> {
        let mut strategies = self.strategies.lock().map_err(|_| "Failed to lock mutex".to_string())?;
        if strategies.remove(&id).is_some() {
            Ok(())
        } else {
            Err("Strategy not found".to_string())
        }
    }
}