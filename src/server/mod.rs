package server

use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use std::io::{self, Read, Write};
use log::{info, error};

struct SocketServer {
    listener: TcpListener,
    thread_pool: Arc<Mutex<Vec<thread::JoinHandle<()>>>>,
    running: Arc<Mutex<bool>>,
}

impl SocketServer {
    fn new(address: &str) -> io::Result<Self> {
        let listener = TcpListener::bind(address)?;
        let thread_pool = Arc::new(Mutex::new(Vec::new()));
        let running = Arc::new(Mutex::new(true));
        Ok(SocketServer { listener, thread_pool, running })
    }

    fn start(&self) {
        let running = Arc::clone(&self.running);
        let listener = self.listener.try_clone().unwrap();
        thread::spawn(move || {
            self.run(listener);
        });
        while *running.lock().unwrap() {
            thread::park();
        }
    }

    fn run(&self, listener: TcpListener) {
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    info!("Accepted connection from {}", stream.peer_addr().unwrap());
                    let thread_pool = Arc::clone(&self.thread_pool);
                    let running = Arc::clone(&self.running);
                    let handle = thread::spawn(move || {
                        Self::handle_connection(stream);
                    });
                    thread_pool.lock().unwrap().push(handle);
                }
                Err(e) => {
                    error!("Error accepting connection: {}", e);
                }
            }
        }
    }

    fn handle_connection(mut stream: TcpStream) {
        let mut buffer = [0; 1024];
        match stream.read(&mut buffer) {
            Ok(size) => {
                stream.write_all(&buffer[0..size]).unwrap();
            }
            Err(e) => {
                error!("Error reading from stream: {}", e);
            }
        }
    }

    fn shutdown(&self) {
        let mut running = self.running.lock().unwrap();
        *running = false;
        for handle in self.thread_pool.lock().unwrap().drain(..) {
            if let Err(e) = handle.join() {
                error!("Error joining thread: {:?}", e);
            }
        }
    }
}