use std::sync::{Arc, Mutex};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Strategy {
    // Define the fields for the Strategy struct
}

lazy_static::lazy_static! {
    static ref GLOBAL_STRATEGY_CONTAINER: Arc<Mutex<HashMap<String, Strategy>>> = Arc::new(Mutex::new(HashMap::new()));
}

pub fn get_global_strategy_container() -> Arc<Mutex<HashMap<String, Strategy>>> {
    GLOBAL_STRATEGY_CONTAINER.clone()
}

pub fn add_strategy(name: String, strategy: Strategy) {
    let mut container = GLOBAL_STRATEGY_CONTAINER.lock().unwrap();
    container.insert(name, strategy);
}

pub fn check_strategy_existence(name: &str) -> bool {
    let container = GLOBAL_STRATEGY_CONTAINER.lock().unwrap();
    container.contains_key(name)
}