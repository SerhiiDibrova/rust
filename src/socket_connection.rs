package socket_connection

use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use std::sync::Arc;
use log::{error, info};

struct SocketConnection {
    stream: TcpStream,
    logger: Arc<dyn Fn(String) + Send + Sync>,
}

impl SocketConnection {
    pub fn new(stream: TcpStream, logger: Arc<dyn Fn(String) + Send + Sync>) -> Self {
        SocketConnection { stream, logger }
    }

    pub async fn write(&self, buffer: &[u8]) -> Result<(), std::io::Error> {
        let encrypted_data = self.encrypt_data(buffer);
        match self.stream.write_all(&encrypted_data).await {
            Ok(_) => {
                (self.logger)(format!("Successfully wrote {} bytes", encrypted_data.len()));
                Ok(())
            }
            Err(e) => {
                error!("Failed to write to socket: {}", e);
                Err(e)
            }
        }
    }

    fn encrypt_data(&self, data: &[u8]) -> Vec<u8> {
        // Implement actual encryption logic here
        data.iter().map(|&b| b ^ 0xFF).collect() // Simple XOR encryption as an example
    }
}