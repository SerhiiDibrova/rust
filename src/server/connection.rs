package server;

use std::io::{self, Read, Write};
use std::net::{TcpStream, Shutdown};

pub struct Connection {
    stream: TcpStream,
    is_connected: bool,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Self {
        Connection {
            stream,
            is_connected: true,
        }
    }

    pub fn read(&mut self) -> io::Result<String> {
        let mut buffer = [0; 1024];
        let bytes_read = self.stream.read(&mut buffer)?;
        if bytes_read == 0 {
            self.is_connected = false;
            return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Connection closed"));
        }
        Ok(String::from_utf8_lossy(&buffer[..bytes_read]).to_string())
    }

    pub fn write(&mut self, response: &str) -> io::Result<()> {
        self.stream.write_all(response.as_bytes()).map_err(|e| {
            self.is_connected = false;
            e
        })?;
        self.stream.flush().map_err(|e| {
            self.is_connected = false;
            e
        })?;
        Ok(())
    }

    pub fn handle_request(&mut self) -> io::Result<()> {
        let request = self.read()?;
        if request.is_empty() {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Empty request"));
        }
        let response = format!("Received: {}", request);
        self.write(&response)?;
        Ok(())
    }

    pub fn close(&mut self) -> io::Result<()> {
        self.is_connected = false;
        self.stream.shutdown(Shutdown::Both)?;
        Ok(())
    }

    pub fn is_connected(&self) -> bool {
        self.is_connected
    }
}