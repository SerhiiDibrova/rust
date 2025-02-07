use serde_json::json;
use std::sync::{Arc, Mutex};
use log::{info, error};

#[derive(Debug)]
struct StockPacket {
    // Define fields for StockPacket
}

impl StockPacket {
    fn validate(&self) -> Result<(), String> {
        // Implement validation logic
        Ok(())
    }
}

struct StockPacketRegistry {
    packets: Arc<Mutex<Vec<String>>>,
}

impl StockPacketRegistry {
    fn new() -> Self {
        StockPacketRegistry {
            packets: Arc::new(Mutex::new(Vec::new())),
        }
    }

    fn register_stock_packets(&self, packet: StockPacket) {
        if let Err(e) = packet.validate() {
            error!("Validation failed: {}", e);
            return;
        }

        match serde_json::to_string(&packet) {
            Ok(serialized) => {
                self.packets.lock().unwrap().push(serialized);
                info!("Registered stock packet: {:?}", packet);
            }
            Err(e) => {
                error!("Serialization failed: {}", e);
            }
        }
    }
}