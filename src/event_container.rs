package src.event_container;

use std::collections::HashMap;

pub struct EventContainer {
    strategies: HashMap<u32, Vec<String>>,
}

impl EventContainer {
    pub fn new() -> Self {
        EventContainer {
            strategies: HashMap::new(),
        }
    }

    pub fn get_strategies(&self, token: u32) -> Option<&Vec<String>> {
        self.strategies.get(&token)
    }

    pub fn contains(&self, token: u32) -> bool {
        self.strategies.contains_key(&token)
    }
}

pub struct MerlinShared {
    pub const _global_event_container: EventContainer = EventContainer::new();
}