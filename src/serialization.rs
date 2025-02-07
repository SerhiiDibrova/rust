use serde_json::json;
use serde_json::to_string;

pub struct TradingData {
    pub symbol: String,
    pub price: f64,
    pub quantity: f64,
    pub comment: Option<String>,
}

pub fn serialize_trading_information(data: &TradingData) -> String {
    if data.symbol.is_empty() || data.price.is_nan() || data.quantity.is_nan() {
        return String::new();
    }

    let mut json_object = json!({
        "symbol": data.symbol,
        "price": data.price,
        "quantity": data.quantity,
    });

    if let Some(ref comment) = data.comment {
        json_object["comment"] = json!(comment);
    }

    to_string(&json_object).unwrap_or_else(|_| String::new())
}