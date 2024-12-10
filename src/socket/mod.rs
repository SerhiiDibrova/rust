package socket

use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use std::io::{self, Read, Write};

struct SocketServer {
    port: u16,
    acceptor: TcpListener,
    thread_pool: Arc<Mutex<Vec<thread::JoinHandle<()>>>>,
}

impl SocketServer {
    fn new(port: u16) -> io::Result<SocketServer> {
        let acceptor = TcpListener::bind(format!("127.0.0.1:{}", port))?;
        let thread_pool = Arc::new(Mutex::new(Vec::new()));
        Ok(SocketServer { port, acceptor, thread_pool })
    }

    fn run_server(&self) {
        println!("Server started on port {}", self.port);
        self.accept_connections();
    }

    fn accept_connections(&self) {
        for stream in self.acceptor.incoming() {
            match stream {
                Ok(stream) => {
                    let thread_pool = Arc::clone(&self.thread_pool);
                    let handle = thread::spawn(move || {
                        handle_connection(stream);
                    });
                    thread_pool.lock().unwrap().push(handle);
                }
                Err(e) => {
                    eprintln!("Error accepting connection: {}", e);
                }
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    match stream.read(&mut buffer) {
        Ok(size) => {
            if let Err(e) = stream.write_all(&buffer[0..size]) {
                eprintln!("Error writing to stream: {}", e);
            }
        }
        Err(e) => {
            eprintln!("Error reading from stream: {}", e);
        }
    }
}