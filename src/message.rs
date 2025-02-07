mod message {
    use std::str::FromStr;
    use std::num::ParseFloatError;

    #[derive(Debug)]
    pub struct Message {
        pub trade_id: String,
        pub quantity: f64,
        pub price: f64,
        pub message_type: String,
    }

    #[derive(Debug)]
    pub enum MessageError {
        InvalidTradeId,
        InvalidQuantity(ParseFloatError),
        InvalidPrice(ParseFloatError),
        InvalidMessageType,
    }

    impl Message {
        pub fn new(trade_id: &str, quantity: &str, price: &str, message_type: &str) -> Result<Self, MessageError> {
            if trade_id.is_empty() {
                return Err(MessageError::InvalidTradeId);
            }
            let quantity = quantity.parse::<f64>().map_err(MessageError::InvalidQuantity)?;
            let price = price.parse::<f64>().map_err(MessageError::InvalidPrice)?;
            if message_type.is_empty() {
                return Err(MessageError::InvalidMessageType);
            }
            Ok(Message {
                trade_id: trade_id.to_string(),
                quantity,
                price,
                message_type: message_type.to_string(),
            })
        }
    }
}