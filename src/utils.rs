use serde::{Serialize, Deserialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
pub struct StockPacket {
    pub symbol: String,
    pub quantity: u32,
    pub price: f64,
}

pub fn serialize_stock_packet(packet: &StockPacket) -> Result<String> {
    serde_json::to_string(packet)
}

pub fn validate_symbol(symbol: &str) -> bool {
    !symbol.is_empty() && symbol.chars().all(|c| c.is_alphanumeric() || c == '_')
}

pub fn validate_quantity(quantity: u32) -> bool {
    quantity > 0
}

pub fn validate_price(price: f64) -> bool {
    price >= 0.0
}