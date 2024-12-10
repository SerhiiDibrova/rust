package main

use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

struct Connection {
    stream: TcpStream,
}

impl Connection {
    fn new(stream: TcpStream) -> Self {
        Connection { stream }
    }

    fn handle_connection(mut self) {
        let mut buffer = [0; 1024];
        match self.stream.read(&mut buffer) {
            Ok(size) => {
                if let Err(e) = self.process_request(&buffer[..size]) {
                    eprintln!("Failed to process request: {}", e);
                } else {
                    if let Err(e) = self.stream.write_all(b"Response sent") {
                        eprintln!("Failed to send response: {}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to read from stream: {}", e);
            }
        }
        if let Err(e) = self.stream.shutdown(std::net::Shutdown::Both) {
            eprintln!("Failed to close connection: {}", e);
        }
    }

    fn process_request(&self, request: &[u8]) -> Result<(), String> {
        String::from_utf8(request.to_vec()).map(|_| ()).map_err(|e| e.to_string())
    }
}

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let connection = Connection::new(stream);
                thread::spawn(move || {
                    connection.handle_connection();
                });
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
    Ok(())
}