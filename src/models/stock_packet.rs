mod stock_packet {
    use regex::Regex;

    pub struct StockPacket {
        order_id: String,
        price: f64,
        fill_price: f64,
        side: String,
        client: String,
        message: String,
        timestamp: String,
    }

    impl StockPacket {
        pub fn new(order_id: String, price: f64, fill_price: f64, side: String, client: String, message: String, timestamp: String) -> Result<Self, String> {
            if order_id.is_empty() {
                return Err("Order ID must not be empty.".to_string());
            }
            if price <= 0.0 {
                return Err("Price must be a positive number.".to_string());
            }
            if fill_price < 0.0 {
                return Err("Fill price can be zero or a positive number.".to_string());
            }
            if side != "buy" && side != "sell" {
                return Err("Side must be either 'buy' or 'sell'.".to_string());
            }
            if client.is_empty() {
                return Err("Client must not be empty.".to_string());
            }
            if message.len() > 256 {
                return Err("Message must not exceed 256 characters.".to_string());
            }
            let iso_regex = Regex::new(r"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}(?:\.\d{1,3})?(?:Z|[+-]\d{2}:\d{2})?$").unwrap();
            if !iso_regex.is_match(&timestamp) {
                return Err("Timestamp must be a valid ISO 8601 formatted string.".to_string());
            }

            Ok(StockPacket {
                order_id,
                price,
                fill_price,
                side,
                client,
                message,
                timestamp,
            })
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_valid_stock_packet() {
            let packet = StockPacket::new(
                "123".to_string(),
                100.0,
                0.0,
                "buy".to_string(),
                "client1".to_string(),
                "This is a message.".to_string(),
                "2023-10-01T12:00:00Z".to_string(),
            );
            assert!(packet.is_ok());
        }

        #[test]
        fn test_empty_order_id() {
            let packet = StockPacket::new(
                "".to_string(),
                100.0,
                0.0,
                "buy".to_string(),
                "client1".to_string(),
                "This is a message.".to_string(),
                "2023-10-01T12:00:00Z".to_string(),
            );
            assert_eq!(packet.err(), Some("Order ID must not be empty.".to_string()));
        }

        #[test]
        fn test_negative_price() {
            let packet = StockPacket::new(
                "123".to_string(),
                -100.0,
                0.0,
                "buy".to_string(),
                "client1".to_string(),
                "This is a message.".to_string(),
                "2023-10-01T12:00:00Z".to_string(),
            );
            assert_eq!(packet.err(), Some("Price must be a positive number.".to_string()));
        }

        #[test]
        fn test_invalid_fill_price() {
            let packet = StockPacket::new(
                "123".to_string(),
                100.0,
                -1.0,
                "buy".to_string(),
                "client1".to_string(),
                "This is a message.".to_string(),
                "2023-10-01T12:00:00Z".to_string(),
            );
            assert_eq!(packet.err(), Some("Fill price can be zero or a positive number.".to_string()));
        }

        #[test]
        fn test_invalid_side() {
            let packet = StockPacket::new(
                "123".to_string(),
                100.0,
                0.0,
                "invalid".to_string(),
                "client1".to_string(),
                "This is a message.".to_string(),
                "2023-10-01T12:00:00Z".to_string(),
            );
            assert_eq!(packet.err(), Some("Side must be either 'buy' or 'sell'.".to_string()));
        }

        #[test]
        fn test_empty_client() {
            let packet = StockPacket::new(
                "123".to_string(),
                100.0,
                0.0,
                "buy".to_string(),
                "".to_string(),
                "This is a message.".to_string(),
                "2023-10-01T12:00:00Z".to_string(),
            );
            assert_eq!(packet.err(), Some("Client must not be empty.".to_string()));
        }

        #[test]
        fn test_message_too_long() {
            let long_message = "a".repeat(257);
            let packet = StockPacket::new(
                "123".to_string(),
                100.0,
                0.0,
                "buy".to_string(),
                "client1".to_string(),
                long_message,
                "2023-10-01T12:00:00Z".to_string(),
            );
            assert_eq!(packet.err(), Some("Message must not exceed 256 characters.".to_string()));
        }

        #[test]
        fn test_invalid_timestamp() {
            let packet = StockPacket::new(
                "123".to_string(),
                100.0,
                0.0,
                "buy".to_string(),
                "client1".to_string(),
                "This is a message.".to_string(),
                "invalid_timestamp".to_string(),
            );
            assert_eq!(packet.err(), Some("Timestamp must be a valid ISO 8601 formatted string.".to_string()));
        }
    }
}