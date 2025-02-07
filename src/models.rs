mod models {
    use serde::Serialize;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[derive(Debug, Clone)]
    pub struct StockPacket {
        pub symbol: String,
        pub price: f64,
        pub quantity: u32,
        pub timestamp: u64,
    }

    impl StockPacket {
        pub fn new(symbol: String, price: f64, quantity: u32) -> Self {
            let timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs();
            StockPacket {
                symbol,
                price,
                quantity,
                timestamp,
            }
        }

        pub fn validate(&self) -> bool {
            !self.symbol.is_empty() && self.price > 0.0 && self.quantity > 0
        }
    }

    #[derive(Debug, Serialize)]
    pub struct EventData {
        pub event_type: String,
        pub event_time: u64,
        pub details: String,
    }

    #[derive(Debug, Serialize)]
    pub struct Strategy {
        pub id: String,
        pub name: String,
        pub description: String,
    }

    #[derive(Serialize)]
    pub struct StrategyResponse {
        pub ids: Vec<String>,
    }

    #[derive(Debug, Clone)]
    pub struct ConnectionIdentifiers {
        pub ip_address: String,
        pub port: u16,
    }

    impl ConnectionIdentifiers {
        pub fn new(ip_address: String, port: u16) -> Self {
            ConnectionIdentifiers { ip_address, port }
        }
    }
}