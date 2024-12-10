package src.server;

use std::net::{TcpListener, TcpStream};
use std::thread;
use std::sync::{Arc, Mutex};
use log::{error, info};
use std::io::{self, Read, Write};

struct SocketServer {
    port: u16,
    listener: TcpListener,
    thread_pool: Arc<Mutex<Vec<thread::JoinHandle<()>>>>,
}

impl SocketServer {
    pub fn new(port: u16) -> Self {
        let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).expect("Could not bind to address");
        SocketServer { port, listener, thread_pool: Arc::new(Mutex::new(Vec::new())) }
    }

    pub fn start_listening(&self) {
        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    match stream.peer_addr() {
                        Ok(addr) => info!("Accepted connection from {}", addr),
                        Err(e) => error!("Error retrieving peer address: {}", e),
                    }
                    let stream = Arc::new(stream);
                    let thread_pool = Arc::clone(&self.thread_pool);
                    let handle = thread::spawn(move || {
                        if let Err(e) = handle_connection(stream) {
                            error!("Error handling connection: {}", e);
                        }
                    });
                    thread_pool.lock().unwrap().push(handle);
                }
                Err(e) => {
                    error!("Error accepting connection: {}", e);
                }
            }
        }
    }
}

fn handle_connection(stream: Arc<TcpStream>) -> io::Result<()> {
    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer)?;
    if bytes_read > 0 {
        stream.write_all(&buffer[..bytes_read])?;
    }
    Ok(())
}