package model;

use serde::{Serialize, Deserialize};
use serde_json::{self, Error};
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize)]
pub struct StrategyStatus {
    id: String,
    name: String,
    status: String,
    last_updated: DateTime<Utc>,
    metrics: Vec<Metric>,
}

#[derive(Serialize, Deserialize)]
pub struct Metric {
    name: String,
    value: f64,
}

impl StrategyStatus {
    pub fn new(id: String, name: String, status: String, last_updated: DateTime<Utc>, metrics: Vec<Metric>) -> Result<Self, String> {
        if id.is_empty() || name.is_empty() || status.is_empty() || metrics.is_empty() {
            return Err("Fields cannot be empty".to_string());
        }
        Ok(StrategyStatus { id, name, status, last_updated, metrics })
    }

    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| e.to_string())
    }

    pub fn from_json(json_str: &str) -> Result<StrategyStatus, String> {
        serde_json::from_str(json_str).map_err(|e| e.to_string())
    }
}