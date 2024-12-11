package models;

use serde::{Serialize, Deserialize};
use serde_json::{Error};

#[derive(Serialize, Deserialize)]
pub struct StockPacket {
    pub symbol: String,
    pub price: f64,
    pub volume: u32,
    pub timestamp: String,
}

impl StockPacket {
    pub fn new(symbol: String, price: f64, volume: u32, timestamp: String) -> Self {
        StockPacket {
            symbol,
            price,
            volume,
            timestamp,
        }
    }

    pub fn to_json(&self) -> Result<String, Error> {
        serde_json::to_string(self).map_err(|e| e.into())
    }

    pub fn from_json(json_str: &str) -> Result<Self, Error> {
        serde_json::from_str(json_str).map_err(|e| e.into())
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.symbol.is_empty() {
            return Err("Symbol cannot be empty".to_string());
        }
        if self.price < 0.0 {
            return Err("Price cannot be negative".to_string());
        }
        if self.volume == 0 {
            return Err("Volume must be greater than zero".to_string());
        }
        Ok(())
    }
}