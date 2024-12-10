package src;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use serde_json::Value;
use std::sync::{Arc, Mutex};
use log::{info, error, Logger};
use std::net::SocketAddr;

struct Connection {
    stream: TcpStream,
    user_id: String,
    buffer: Vec<u8>,
    order_manager: Arc<Mutex<OrderManager>>,
    logger: Logger,
}

impl Connection {
    pub fn new(stream: TcpStream, user_id: String, order_manager: Arc<Mutex<OrderManager>>, logger: Logger) -> Self {
        let addr: SocketAddr = stream.peer_addr().expect("Unable to get peer address");
        stream.set_nodelay(true).expect("Unable to set socket options");
        Self {
            stream,
            user_id,
            buffer: vec![0; 1024],
            order_manager,
            logger,
        }
    }

    pub async fn read(&mut self) {
        loop {
            match self.stream.read(&mut self.buffer).await {
                Ok(bytes_read) if bytes_read > 0 => {
                    self.handle_read(&self.buffer[..bytes_read]).await;
                }
                Ok(_) => {}
                Err(e) => {
                    error!("Error reading from stream: {:?}", e);
                    break;
                }
            }
        }
    }

    pub async fn write_async(&mut self, data: &[u8], size: usize) {
        if let Err(e) = self.stream.write_all(&data[..size]).await {
            error!("Error writing to stream: {:?}", e);
        } else {
            info!("Wrote {} bytes to the client", size);
        }
    }

    async fn handle_read(&mut self, data: &[u8]) {
        match self.deserialize_request(data) {
            Ok(request) => {
                if self.validate_request(&request) {
                    self.process_query(request).await;
                } else {
                    error!("Invalid JSON request");
                    self.write_async(b"Invalid JSON request", 20).await;
                }
            }
            Err(e) => {
                error!("Error processing read: {:?}", e);
            }
        }
        self.read().await;
    }

    fn deserialize_request(&self, data: &[u8]) -> Result<Value, serde_json::Error> {
        serde_json::from_slice(data)
    }

    fn validate_request(&self, request: &Value) -> bool {
        request.is_object() && request.get("type").is_some()
    }

    async fn process_query(&mut self, request: Value) {
        let request_type = request.get("type").and_then(Value::as_str).unwrap_or("");
        match request_type {
            "LOGIN" => self.handle_login(request).await,
            "NEW" => self.handle_new_order(request).await,
            "MODIFY" => self.handle_modify_order(request).await,
            "CANCEL" => self.handle_cancel_order(request).await,
            "SUBSCRIBE" => self.handle_subscribe(request).await,
            "APPLY" => self.handle_apply(request).await,
            "UNSUBSCRIBE" => self.handle_unsubscribe(request).await,
            "SUBSCRIBE_APPLY" => self.handle_subscribe_apply(request).await,
            _ => {
                error!("Unsupported request type: {}", request_type);
                self.write_async(b"Unsupported request type", 24).await;
            }
        }
    }

    async fn handle_login(&self, request: Value) {
        info!("Processing login request: {:?}", request);
    }

    async fn handle_new_order(&self, request: Value) {
        info!("Processing new order request: {:?}", request);
    }

    async fn handle_modify_order(&self, request: Value) {
        info!("Processing modify order request: {:?}", request);
    }

    async fn handle_cancel_order(&self, request: Value) {
        info!("Processing cancel order request: {:?}", request);
    }

    async fn handle_subscribe(&self, request: Value) {
        info!("Processing subscribe request: {:?}", request);
    }

    async fn handle_apply(&self, request: Value) {
        info!("Processing apply request: {:?}", request);
    }

    async fn handle_unsubscribe(&self, request: Value) {
        info!("Processing unsubscribe request: {:?}", request);
    }

    async fn handle_subscribe_apply(&self, request: Value) {
        info!("Processing subscribe apply request: {:?}", request);
    }

    pub async fn handle_connection(&mut self) {
        self.read().await;
    }
}