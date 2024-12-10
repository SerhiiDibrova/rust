package server;

use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::{self, Read, Write};
use std::sync::{Arc, Mutex};
use std::time::Duration;

struct SocketServer {
    listener: TcpListener,
    running: Arc<Mutex<bool>>,
}

impl SocketServer {
    fn new(port: u16) -> io::Result<Self> {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", port))?;
        let running = Arc::new(Mutex::new(true));
        Ok(SocketServer { listener, running })
    }

    fn start_accept(&self) {
        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    let running = Arc::clone(&self.running);
                    thread::spawn(move || {
                        handle_client(stream);
                    });
                }
                Err(e) => {
                    eprintln!("Error accepting connection: {}", e);
                }
            }
        }
    }

    fn run_server(&self) {
        self.start_accept();
        let running = Arc::clone(&self.running);
        while *running.lock().unwrap() {
            thread::sleep(Duration::from_millis(100));
        }
    }

    fn stop_server(&self) {
        let mut running = self.running.lock().unwrap();
        *running = false;
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    match stream.read(&mut buffer) {
        Ok(_) => {
            if let Err(e) = stream.write(&buffer) {
                eprintln!("Error writing to stream: {}", e);
            }
        }
        Err(e) => {
            eprintln!("Error reading from stream: {}", e);
        }
    }
}