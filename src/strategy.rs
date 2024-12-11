package src.strategy;

use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use log::{error, info};

const MAX_CAPACITY: usize = 100;
const MAX_QUEUE_SIZE: usize = 100;

pub struct StockPacket;

pub struct Strategy {
    _activated: bool,
    _address: u32,
    event_container: Arc<Mutex<HashMap<u32, Vec<Arc<Strategy>>>>,
    message_queue: Arc<Mutex<Vec<StockPacket>>>,
}

impl Strategy {
    pub fn new(activated: bool, address: u32, event_container: Arc<Mutex<HashMap<u32, Vec<Arc<Strategy>>>>, message_queue: Arc<Mutex<Vec<StockPacket>>>) -> Self {
        Strategy {
            _activated: activated,
            _address: address,
            event_container,
            message_queue,
        }
    }

    pub fn register_for_data(&self, token: u32) {
        if token == 0 {
            return;
        }
        info!("Registering strategy at address: {} for token: {}", self._address, token);
        let mut container = self.event_container.lock().unwrap();
        let entry = container.entry(token).or_insert_with(Vec::new);
        if !entry.contains(&Arc::new(self.clone())) {
            entry.push(Arc::new(self.clone()));
        }
    }

    pub fn register_self(&self) {
        let self_arc = Arc::new(self.clone());
        let mut container = self.event_container.lock().unwrap();
        if !container.values().any(|strategies| strategies.contains(&self_arc)) {
            if container.len() < MAX_CAPACITY {
                container.insert(self._address, vec![self_arc]);
            } else {
                error!("Cannot register strategy: maximum capacity reached.");
            }
        } else {
            error!("Strategy at address: {} is already registered.", self._address);
        }
    }

    pub fn activated(&self) -> bool {
        self._activated
    }

    pub fn set_activated(&mut self, activated: bool) {
        self._activated = activated;
    }

    pub fn get_address(&self) -> u32 {
        self._address
    }

    pub fn update_arthur(&self, packet: StockPacket) {
        if let Ok(mut queue) = self.message_queue.lock() {
            if queue.len() < MAX_QUEUE_SIZE {
                queue.push(packet);
            } else {
                error!("Message queue is full, cannot push packet.");
            }
        }
    }
}