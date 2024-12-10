package src;

use serde_json::json;
use std::time::{SystemTime, UNIX_EPOCH};

const JSON_PF_NUMBER: &str = "strategy_number";
const JSON_UNIQUE_ID: &str = "unique_id";
const JSON_TOKEN: &str = "token";
const JSON_QUANTITY: &str = "total_quantity";
const JSON_FILL_QUANTITY: &str = "filled_quantity";
const JSON_REMAINING: &str = "remaining_quantity";
const JSON_ORDER_ID: &str = "order_id";
const JSON_PRICE: &str = "adjusted_price";
const JSON_FILL_PRICE: &str = "last_trade_price";
const JSON_SIDE: &str = "trade_side";
const JSON_CLIENT: &str = "client_code";
const JSON_MESSAGE: &str = "contract_description";
const JSON_TIME: &str = "timestamp";

pub struct StockPacket {
    pub strategy_number: String,
    pub unique_id: String,
    pub token: String,
    pub total_quantity: u32,
    pub filled_quantity: u32,
    pub order_id: String,
    pub price: u32,
    pub last_trade_price: u32,
    pub trade_side: String,
    pub client_code: String,
    pub contract_description: String,
}

fn get_current_time() -> String {
    let start = SystemTime::now();
    let duration = start.duration_since(UNIX_EPOCH).unwrap();
    let seconds = duration.as_secs();
    let nanos = duration.subsec_nanos();
    format!("{}.{:09}", seconds, nanos)
}

pub fn serialize_stock_packet(stock_packet: &StockPacket) -> Result<String, String> {
    if stock_packet.is_null() {
        return Err("Invalid stock packet: null reference".to_string());
    }
    if stock_packet.filled_quantity > stock_packet.total_quantity {
        return Err("Invalid stock packet: filled quantity exceeds total quantity".to_string());
    }
    let remaining_quantity = stock_packet.total_quantity - stock_packet.filled_quantity;
    let json_object = json!({
        JSON_PF_NUMBER: stock_packet.strategy_number,
        JSON_UNIQUE_ID: stock_packet.unique_id,
        JSON_TOKEN: stock_packet.token,
        JSON_QUANTITY: stock_packet.total_quantity,
        JSON_FILL_QUANTITY: stock_packet.filled_quantity,
        JSON_REMAINING: remaining_quantity,
        JSON_ORDER_ID: stock_packet.order_id,
        JSON_PRICE: stock_packet.price as f64 / 100.0,
        JSON_FILL_PRICE: stock_packet.last_trade_price as f64 / 100.0,
        JSON_SIDE: stock_packet.trade_side,
        JSON_CLIENT: stock_packet.client_code,
        JSON_MESSAGE: stock_packet.contract_description,
        JSON_TIME: get_current_time(),
    });
    serde_json::to_string(&json_object).map_err(|e| e.to_string())
}