package models

use serde::{Serialize, Deserialize};
use serde_json::Error;

#[derive(Serialize, Deserialize)]
pub struct StockPacket {
    token: String,
    ioc: bool,
    price: f64,
    quantity: u32,
}

impl StockPacket {
    pub fn new(token: String, ioc: bool, price: f64, quantity: u32) -> Self {
        StockPacket { token, ioc, price, quantity }
    }

    pub fn set_token(&mut self, token: String) {
        if !token.is_empty() {
            self.token = token;
        }
    }

    pub fn set_ioc(&mut self, ioc: bool) {
        self.ioc = ioc;
    }

    pub fn set_price(&mut self, price: f64) {
        if price >= 0.0 {
            self.price = price;
        }
    }

    pub fn set_quantity(&mut self, quantity: u32) {
        self.quantity = quantity;
    }

    pub fn get_token(&self) -> &String {
        &self.token
    }

    pub fn get_ioc(&self) -> bool {
        self.ioc
    }

    pub fn get_price(&self) -> f64 {
        self.price
    }

    pub fn get_quantity(&self) -> u32 {
        self.quantity
    }

    pub fn to_json(&self) -> Result<String, Error> {
        serde_json::to_string(self).map_err(|e| e)
    }

    pub fn from_json(json: &str) -> Result<Self, Error> {
        serde_json::from_str(json).map_err(|e| e)
    }
}