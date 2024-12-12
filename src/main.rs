use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use std::vec::Vec;

struct ThreadPool {
    workers: Vec<Worker>,
    sender: std::sync::mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    fn new(size: usize) -> ThreadPool {
        let (sender, receiver) = std::sync::mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);

        for _ in 0..size {
            workers.push(Worker::new(Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

struct Worker {
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(receiver: Arc<Mutex<std::sync::mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();
            job();
        });

        Worker {
            thread: Some(thread),
        }
    }
}

struct WebServer {
    address: String,
    thread_pool: ThreadPool,
}

impl WebServer {
    fn new(address: &str, pool_size: usize) -> WebServer {
        WebServer {
            address: address.to_string(),
            thread_pool: ThreadPool::new(pool_size),
        }
    }

    fn start(&self) {
        let listener = TcpListener::bind(&self.address).unwrap();
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let stream = stream.try_clone().unwrap();
                    self.thread_pool.execute(move || {
                        handle_request(stream);
                    });
                }
                Err(e) => {
                    eprintln!("Error accepting connection: {}", e);
                }
            }
        }
    }
}

fn handle_request(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    if let Err(e) = stream.read(&mut buffer) {
        eprintln!("Error reading from stream: {}", e);
        return;
    }

    let request = String::from_utf8_lossy(&buffer);
    if request.starts_with("GET") {
        let response = "HTTP/1.1 200 OK\r\n\r\nHello, World!";
        if let Err(e) = stream.write(response.as_bytes()) {
            eprintln!("Error writing to stream: {}", e);
        }
    } else if request.starts_with("POST") {
        let response = "HTTP/1.1 200 OK\r\n\r\nPost request received!";
        if let Err(e) = stream.write(response.as_bytes()) {
            eprintln!("Error writing to stream: {}", e);
        }
    } else {
        let response = "HTTP/1.1 400 Bad Request\r\n\r\n";
        if let Err(e) = stream.write(response.as_bytes()) {
            eprintln!("Error writing to stream: {}", e);
        }
    }
}

fn main() {
    let server = WebServer::new("127.0.0.1:7878", 4);
    server.start();
}