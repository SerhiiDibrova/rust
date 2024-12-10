package socket

use std::io::{self, Read, Write};
use std::net::{TcpStream, Shutdown};
use log::{error};

pub struct Connection {
    socket: TcpStream,
}

impl Connection {
    pub fn new(socket: TcpStream) -> Connection {
        Connection { socket }
    }

    pub fn handle_connection(&mut self) {
        let mut buffer = [0; 1024];
        loop {
            match self.socket.read(&mut buffer) {
                Ok(0) => {
                    break;
                }
                Ok(size) => {
                    let command = String::from_utf8_lossy(&buffer[..size]);
                    let response = self.process_command(&command);
                    if let Err(e) = self.socket.write_all(response.as_bytes()) {
                        error!("Failed to write to socket: {}", e);
                        break;
                    }
                }
                Err(e) => {
                    error!("Failed to read from socket: {}", e);
                    break;
                }
            }
        }
        if let Err(e) = self.socket.shutdown(Shutdown::Both) {
            error!("Failed to shutdown socket: {}", e);
        }
    }

    fn process_command(&self, command: &str) -> String {
        format!("Received: {}", command)
    }
}