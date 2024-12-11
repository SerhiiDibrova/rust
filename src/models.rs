package models

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct StockPacket {
    pub stock_symbol: String,
    pub quantity: u32,
    pub price: f64,
}

impl StockPacket {
    pub fn validate(&self) -> Result<(), String> {
        if self.stock_symbol.is_empty() {
            return Err("Stock symbol cannot be empty".to_string());
        }
        if self.quantity == 0 {
            return Err("Quantity must be greater than zero".to_string());
        }
        if self.price < 0.0 {
            return Err("Price cannot be negative".to_string());
        }
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderResponse {
    pub order_id: String,
    pub stock_symbol: String,
    pub quantity: u32,
    pub price: f64,
    pub timestamp: DateTime<Utc>,
    pub status: OrderStatus,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum OrderStatus {
    Pending,
    Completed,
    Canceled,
    Failed,
}

impl OrderResponse {
    pub fn validate(&self) -> Result<(), String> {
        if self.order_id.is_empty() {
            return Err("Order ID cannot be empty".to_string());
        }
        if self.stock_symbol.is_empty() {
            return Err("Stock symbol cannot be empty".to_string());
        }
        if self.quantity == 0 {
            return Err("Quantity must be greater than zero".to_string());
        }
        if self.price < 0.0 {
            return Err("Price cannot be negative".to_string());
        }
        if self.timestamp.timestamp() <= 0 {
            return Err("Timestamp must be a valid positive value".to_string());
        }
        Ok(())
    }
}