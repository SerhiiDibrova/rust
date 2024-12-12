mod json_utils {
    use serde_json::json;
    use serde_json::Error as SerdeError;

    pub struct StockPacket {
        pub symbol: String,
        pub price: f64,
        pub volume: u32,
        pub timestamp: String,
    }

    pub fn escape_json(input: &str) -> Result<String, String> {
        if input.is_empty() {
            return Err("Input string is empty".to_string());
        }
        let mut escaped = String::new();
        for c in input.chars() {
            match c {
                '"' => escaped.push_str("\\\""),
                '\\' => escaped.push_str("\\\\"),
                '/' => escaped.push_str("\\/"),
                '\b' => escaped.push_str("\\b"),
                '\f' => escaped.push_str("\\f"),
                '\n' => escaped.push_str("\\n"),
                '\r' => escaped.push_str("\\r"),
                '\t' => escaped.push_str("\\t"),
                _ => escaped.push(c),
            }
        }
        Ok(escaped)
    }

    pub fn construct_json(stock_packet: &StockPacket) -> Result<String, SerdeError> {
        if stock_packet.price.is_nan() || stock_packet.price.is_infinite() {
            return Err(SerdeError::custom("Price must be a valid number"));
        }
        let escaped_symbol = escape_json(&stock_packet.symbol).map_err(|e| SerdeError::custom(format!("Failed to escape symbol: {}", e)))?;
        let escaped_timestamp = escape_json(&stock_packet.timestamp).map_err(|e| SerdeError::custom(format!("Failed to escape timestamp: {}", e)))?;
        let json_object = json!({
            "symbol": escaped_symbol,
            "price": stock_packet.price,
            "volume": stock_packet.volume,
            "timestamp": escaped_timestamp,
        });
        json_object.to_string().map_err(|e| SerdeError::custom(format!("JSON serialization error: {}", e)))
    }
}