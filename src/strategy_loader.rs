package strategy_loader;

use std::fs;
use std::sync::{Arc, Mutex};
use log::warn;
use libloading::{Library, Symbol};
use std::collections::HashMap;

struct StrategyContainer {
    strategies: Arc<Mutex<HashMap<String, Box<dyn Strategy>>>>,
}

impl StrategyContainer {
    fn new() -> Self {
        StrategyContainer {
            strategies: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn load_strategy(&self, file_name: &str, address: &str, params: &str) -> Result<(), String> {
        if !fs::metadata(file_name).is_ok() {
            warn!("Strategy file does not exist: {}", file_name);
            return Err("File not found".to_string());
        }

        let lib = Library::new(file_name).map_err(|_| {
            warn!("Failed to load library: {}", file_name);
            "Failed to load library".to_string()
        })?;

        unsafe {
            let func: Symbol<unsafe extern fn(&str) -> Box<dyn Strategy>> = lib.get(b"getObject").map_err(|_| {
                warn!("Entry point function 'getObject' not found in library: {}", file_name);
                "Entry point not found".to_string()
            })?;

            let strategy = func(params);
            self.strategies.lock().map_err(|_| {
                warn!("Failed to lock strategies");
                "Failed to lock strategies".to_string()
            })?.insert(address.to_string(), strategy);
        }

        Ok(())
    }

    fn update_strategy_parameters(&self, identifier: &str, new_params: &str) -> Result<(), String> {
        let mut strategies = self.strategies.lock().map_err(|_| {
            warn!("Failed to lock strategies");
            "Failed to lock strategies".to_string()
        })?;
        if let Some(strategy) = strategies.get_mut(identifier) {
            strategy.update_parameters(new_params);
            return Ok(());
        }
        Err("Strategy not found".to_string())
    }

    fn stop_strategy(&self, address: &str) -> Result<(), String> {
        let mut strategies = self.strategies.lock().map_err(|_| {
            warn!("Failed to lock strategies");
            "Failed to lock strategies".to_string()
        })?;
        if let Some(strategy) = strategies.remove(address) {
            strategy.stop();
            return Ok(());
        }
        Err("Strategy not found".to_string())
    }
}

trait Strategy {
    fn update_parameters(&mut self, params: &str);
    fn stop(&mut self);
}