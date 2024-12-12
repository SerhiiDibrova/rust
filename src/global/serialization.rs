use serde::{Serialize, Deserialize};
use serde_json;
use std::fmt;

#[derive(Serialize, Deserialize)]
struct StockPacket {
    order_id: String,
    price: f64,
    fill_price: f64,
    side: String,
    client_id: String,
    message_content: String,
    timestamp: String,
}

#[derive(Debug)]
enum SerializationError {
    MissingField(String),
    SerdeError(serde_json::Error),
}

impl fmt::Display for SerializationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SerializationError::MissingField(field) => write!(f, "Missing required field: {}", field),
            SerializationError::SerdeError(err) => write!(f, "Serialization error: {}", err),
        }
    }
}

impl From<serde_json::Error> for SerializationError {
    fn from(err: serde_json::Error) -> Self {
        SerializationError::SerdeError(err)
    }
}

fn serialize_stock_packet(packet: &StockPacket) -> Result<String, SerializationError> {
    if packet.order_id.is_empty() {
        return Err(SerializationError::MissingField("order_id".to_string()));
    }
    if packet.price.is_nan() {
        return Err(SerializationError::MissingField("price".to_string()));
    }
    if packet.fill_price.is_nan() {
        return Err(SerializationError::MissingField("fill_price".to_string()));
    }
    if packet.side.is_empty() {
        return Err(SerializationError::MissingField("side".to_string()));
    }
    if packet.client_id.is_empty() {
        return Err(SerializationError::MissingField("client_id".to_string()));
    }
    if packet.message_content.is_empty() {
        return Err(SerializationError::MissingField("message_content".to_string()));
    }
    if packet.timestamp.is_empty() {
        return Err(SerializationError::MissingField("timestamp".to_string()));
    }
    serde_json::to_string(packet).map_err(SerializationError::from)
}