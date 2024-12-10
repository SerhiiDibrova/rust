package strategy_param_update

use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use log::{info, warn};

#[derive(Deserialize)]
struct StrategyParamUpdateRequest {
    strategy_id: String,
    parameters: HashMap<String, String>,
}

struct Global {
    strategies: HashMap<String, Box<dyn Strategy>>,
}

impl Global {
    fn get_strategy(&self, strategy_id: &str) -> Option<&Box<dyn Strategy>> {
        self.strategies.get(strategy_id)
    }
}

trait Strategy {
    fn update_parameters(&mut self, parameters: HashMap<String, String>) -> Result<(), String>;
}

fn handle_strategy_param_update(request: &str, global: &mut Global) -> String {
    match serde_json::from_str::<StrategyParamUpdateRequest>(request) {
        Ok(update_request) => {
            if let Some(strategy) = global.get_strategy(&update_request.strategy_id) {
                match strategy.update_parameters(update_request.parameters) {
                    Ok(_) => {
                        info!("Successfully updated parameters for strategy: {}", update_request.strategy_id);
                        json!({"status": "success"}).to_string()
                    }
                    Err(err) => {
                        warn!("Error updating parameters for strategy {}: {}", update_request.strategy_id, err);
                        json!({"status": "error", "message": "Failed to update parameters"}).to_string()
                    }
                }
            } else {
                warn!("Strategy not found: {}", update_request.strategy_id);
                json!({"status": "error", "message": "Strategy not found"}).to_string()
            }
        }
        Err(e) => {
            warn!("Failed to deserialize request: {}. Error: {}", request, e);
            json!({"status": "error", "message": "Invalid request format"}).to_string()
        }
    }
}