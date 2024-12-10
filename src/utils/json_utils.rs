package utils;

use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize)]
pub struct StrategyStatus {
    pub status: String,
    pub message: String,
}

pub fn serialize_strategy_status(status: &StrategyStatus) -> Result<String, String> {
    serde_json::to_string(status).map_err(|e| format!("Serialization error: {}", e))
}