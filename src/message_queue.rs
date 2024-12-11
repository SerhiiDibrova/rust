package message_queue

use std::sync::{Arc, Mutex};
use std::collections::VecDeque;

pub struct StockPacket {
    pub symbol: String,
    pub quantity: u32,
    pub price: f64,
}

pub struct MessageQueue {
    queue: Arc<Mutex<VecDeque<StockPacket>>>,
    capacity: usize,
}

impl MessageQueue {
    pub fn new(capacity: usize) -> Self {
        MessageQueue {
            queue: Arc::new(Mutex::new(VecDeque::with_capacity(capacity))),
            capacity,
        }
    }

    pub fn write_available(&self) -> bool {
        let queue = self.queue.lock().unwrap();
        queue.len() < self.capacity
    }

    pub fn push(&self, stock_packet: StockPacket) -> Result<(), &'static str> {
        let mut queue = self.queue.lock().unwrap();
        if queue.len() < self.capacity {
            queue.push_back(stock_packet);
            Ok(())
        } else {
            Err("Cannot push: Queue is full")
        }
    }

    pub fn pop(&self) -> Option<StockPacket> {
        let mut queue = self.queue.lock().unwrap();
        if queue.is_empty() {
            None
        } else {
            queue.pop_front()
        }
    }

    pub fn size(&self) -> usize {
        let queue = self.queue.lock().unwrap();
        queue.len()
    }

    pub fn clear(&self) -> bool {
        let mut queue = self.queue.lock().unwrap();
        queue.clear();
        true
    }
}