package event_handler

use std::collections::HashMap;
use log::{error, info};

struct Strategy {
    activated: bool,
    market_event_manager: fn(&str) -> Result<(), String>,
}

struct EventHandler {
    event_container: HashMap<String, Vec<Strategy>>,
}

impl EventHandler {
    fn new() -> Self {
        EventHandler {
            event_container: HashMap::new(),
        }
    }

    fn receive_event(&self, token: &str) -> Result<(), String> {
        if token.is_empty() {
            return Err("Token cannot be an empty string".to_string());
        }
        if let Some(strategies) = self.event_container.get(token) {
            let mut errors = Vec::new();
            let unique_strategies: Vec<_> = strategies.iter().cloned().collect();
            for strategy in unique_strategies {
                if strategy.activated {
                    if let Err(e) = (strategy.market_event_manager)(token) {
                        errors.push(e);
                    }
                }
            }
            if errors.is_empty() {
                Ok(())
            } else {
                Err(errors.join(", "))
            }
        } else {
            let error_message = format!("Token '{}' not found in event container", token);
            error!("{}", error_message);
            Err(error_message)
        }
    }

    fn register_event(&mut self, token: String, strategy: Strategy) -> Result<(), String> {
        if !strategy.activated {
            return Err("Strategy must be activated to register".to_string());
        }
        self.event_container
            .entry(token)
            .or_insert_with(Vec::new)
            .push(strategy);
        Ok(())
    }
}