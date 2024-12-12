use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::str;

pub struct Connection {
    stream: TcpStream,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Self {
        Connection { stream }
    }

    pub fn handle(&mut self) -> io::Result<()> {
        let mut buffer = [0; 1024];
        let bytes_read = self.stream.read(&mut buffer).map_err(|e| {
            eprintln!("Failed to read from stream: {}", e);
            e
        })?;

        if bytes_read == 0 {
            eprintln!("Connection closed by client");
            return Ok(());
        }

        let request = str::from_utf8(&buffer[..bytes_read]).map_err(|e| {
            eprintln!("Failed to parse request: {}", e);
            io::Error::new(io::ErrorKind::InvalidData, e)
        })?;

        let response = format!("Received: {}", request);
        self.stream.write_all(response.as_bytes()).map_err(|e| {
            eprintln!("Failed to send response: {}", e);
            e
        })?;

        Ok(())
    }
}