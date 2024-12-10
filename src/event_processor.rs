package src.event_processor;

use std::collections::HashMap;

pub struct Strategy {
    activated: bool,
}

impl Strategy {
    pub fn market_event_manager(&self, token: &str) {
        // Implementation of market_event_manager
    }

    pub fn is_activated(&self) -> bool {
        self.activated
    }
}

pub struct EventProcessor {
    event_container: HashMap<String, Vec<Strategy>>,
}

impl EventProcessor {
    pub fn new() -> Self {
        EventProcessor {
            event_container: HashMap::new(),
        }
    }

    pub fn event_receiver(&self, token: String) {
        if let Some(strategies) = self.event_container.get(&token) {
            for strategy in strategies {
                if strategy.is_activated() {
                    strategy.market_event_manager(&token);
                }
            }
        } else {
            eprintln!("Token not found: {}", token);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_receiver_with_existing_token() {
        let mut processor = EventProcessor::new();
        let strategy = Strategy { activated: true };
        processor.event_container.insert("token1".to_string(), vec![strategy]);

        processor.event_receiver("token1".to_string());
        // Add assertions to validate the expected behavior
    }

    #[test]
    fn test_event_receiver_with_non_existing_token() {
        let processor = EventProcessor::new();
        processor.event_receiver("non_existing_token".to_string());
        // Add assertions to validate the expected behavior
    }

    #[test]
    fn test_event_receiver_with_non_activated_strategy() {
        let mut processor = EventProcessor::new();
        let strategy = Strategy { activated: false };
        processor.event_container.insert("token1".to_string(), vec![strategy]);

        processor.event_receiver("token1".to_string());
        // Add assertions to validate the expected behavior
    }
}