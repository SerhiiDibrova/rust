package strategy_manager;

import std::collections::HashMap;
import std::sync::{Arc, Mutex};
import log::{warn, LevelFilter};
import log::Log;

struct Strategy {
    name: String,
    active: bool,
}

struct StrategyManager {
    strategies: Arc<Mutex<HashMap<u32, Strategy>>>,
}

impl StrategyManager {
    fn new() -> Self {
        setup_logging();
        StrategyManager {
            strategies: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn get_strategy(&self, address: u32) -> Option<Strategy> {
        let strategies = self.strategies.lock().unwrap();
        strategies.get(&address).cloned()
    }

    fn stop_strategy(&self, address: u32) {
        let mut strategies = self.strategies.lock().unwrap();
        if let Some(strategy) = strategies.get_mut(&address) {
            strategy.stop_event_manager();
        } else {
            warn!("Failed to stop strategy: Strategy with address {} not found.", address);
        }
    }
}

impl Strategy {
    fn stop_event_manager(&mut self) {
        self.active = false;
        // Logic to stop the event manager
    }
}

fn setup_logging() {
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(LevelFilter::Warn);
}

struct LOGGER;

impl Log for LOGGER {
    fn enabled(&self, _: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        println!("{}: {}", record.level(), record.args());
    }

    fn flush(&self) {}
}