use std::net::{TcpStream, SocketAddr};
use std::sync::{Arc, Mutex};
use std::io::{self, Read, Write};

pub struct Connection {
    pub socket: TcpStream,
    pub user_data: Arc<Mutex<String>>,
    pub ip_address: String,
    pub port: u16,
    pub status: String,
}

impl Connection {
    pub fn new(socket: TcpStream, user_data: Arc<Mutex<String>>, ip_address: String, port: u16, status: String) -> Self {
        Connection {
            socket,
            user_data,
            ip_address,
            port,
            status,
        }
    }

    pub fn send_data(&mut self, data: &[u8]) -> io::Result<usize> {
        self.socket.write(data)
    }

    pub fn receive_data(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        self.socket.read(buffer)
    }

    pub fn validate_connection(&self) -> bool {
        !self.ip_address.is_empty() && self.port > 0
    }

    pub fn process_incoming_data(&mut self) -> io::Result<usize> {
        let mut buffer = [0; 1024];
        let bytes_read = self.receive_data(&mut buffer)?;
        // Process the data as needed
        Ok(bytes_read)
    }
}