mod global {
    use serde_json::json;
    use log::{error};

    #[derive(Debug)]
    pub struct StockPacket {
        pub order_id: String,
        pub price: f64,
        pub fill_price: f64,
        pub side: String,
        pub client: String,
        pub message: String,
        pub timestamp: String,
    }

    pub struct Global;

    impl Global {
        pub fn serialize(stock_packet: &Option<Box<StockPacket>>) -> String {
            if stock_packet.is_none() {
                return "{}".to_string();
            }

            let packet = stock_packet.as_ref().unwrap();

            if packet.order_id.is_empty() || packet.side.is_empty() || packet.client.is_empty() || 
               packet.message.is_empty() || packet.timestamp.is_empty() || 
               packet.price.is_nan() || packet.fill_price.is_nan() || 
               packet.price < 0.0 || packet.fill_price < 0.0 || 
               !packet.timestamp.chars().all(char::is_numeric) {
                error!("Invalid stock packet data: {:?}", packet);
                return json!({"error": "Invalid stock packet data"}).to_string();
            }

            let json_string = json!({
                "order_id": escape_json(&packet.order_id),
                "price": format!("{:.2}", packet.price).trim_end_matches('0').trim_end_matches('.'),
                "fill_price": format!("{:.2}", packet.fill_price).trim_end_matches('0').trim_end_matches('.'),
                "side": escape_json(&packet.side),
                "client": escape_json(&packet.client),
                "message": escape_json(&packet.message),
                "timestamp": escape_json(&packet.timestamp),
            });

            json_string.to_string()
        }
    }

    fn escape_json(input: &str) -> String {
        let mut escaped = String::new();
        for c in input.chars() {
            match c {
                '"' => escaped.push_str("\\\""),
                '\\' => escaped.push_str("\\\\"),
                _ => escaped.push(c),
            }
        }
        escaped
    }
}