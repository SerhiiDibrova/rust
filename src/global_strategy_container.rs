package global_strategy_container

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;

pub struct Strategy;

pub struct GlobalStrategyContainer {
    strategies: Mutex<HashMap<String, Arc<Strategy>>>,
}

impl GlobalStrategyContainer {
    pub fn new() -> Self {
        GlobalStrategyContainer {
            strategies: Mutex::new(HashMap::new()),
        }
    }

    pub fn add_strategy(&self, name: String, strategy: Arc<Strategy>) {
        if let Ok(mut strategies) = self.strategies.lock() {
            strategies.insert(name, strategy);
        }
    }

    pub fn has_strategy(&self, name: &str) -> bool {
        if let Ok(strategies) = self.strategies.lock() {
            strategies.contains_key(name)
        } else {
            false
        }
    }

    pub fn remove_strategy(&self, name: &str) {
        if let Ok(mut strategies) = self.strategies.lock() {
            strategies.remove(name);
        }
    }
}

lazy_static! {
    pub static ref GLOBAL_STRATEGY_CONTAINER: GlobalStrategyContainer = GlobalStrategyContainer::new();
}