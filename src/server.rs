mod server {
    use std::io::{Read, Write};
    use std::net::{TcpListener, TcpStream};
    use std::sync::{Arc, Mutex};
    use std::thread;

    struct ThreadPool {
        workers: Vec<Worker>,
        sender: Arc<Mutex<std::sync::mpsc::Sender<Job>>>,
    }

    type Job = Box<dyn FnOnce() + Send + 'static>;

    struct Worker {
        id: usize,
        thread: Option<thread::JoinHandle<()>>,
    }

    impl ThreadPool {
        fn new(size: usize) -> ThreadPool {
            let (sender, receiver) = std::sync::mpsc::channel();
            let receiver = Arc::new(Mutex::new(receiver));
            let mut workers = Vec::with_capacity(size);

            for id in 0..size {
                workers.push(Worker::new(id, Arc::clone(&receiver)));
            }

            ThreadPool { workers, sender: Arc::new(Mutex::new(sender)) }
        }

        fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static,
        {
            let job = Box::new(f);
            let sender = self.sender.lock().unwrap();
            sender.send(job).unwrap();
        }
    }

    impl Worker {
        fn new(id: usize, receiver: Arc<Mutex<std::sync::mpsc::Receiver<Job>>>) -> Worker {
            let thread = thread::spawn(move || loop {
                let job = receiver.lock().unwrap().recv().unwrap();
                job();
            });

            Worker { id, thread: Some(thread) }
        }
    }

    pub struct WebServer {
        address: String,
        thread_pool: Arc<ThreadPool>,
    }

    impl WebServer {
        pub fn new(address: &str, pool_size: usize) -> Self {
            let thread_pool = Arc::new(ThreadPool::new(pool_size));
            WebServer { address: address.to_string(), thread_pool }
        }

        pub fn start(&self) {
            let listener = TcpListener::bind(&self.address).unwrap();
            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => {
                        let thread_pool = Arc::clone(&self.thread_pool);
                        thread_pool.execute(move || {
                            handle_connection(stream);
                        });
                    }
                    Err(e) => eprintln!("Failed to accept connection: {}", e),
                }
            }
        }
    }

    fn handle_connection(mut stream: TcpStream) {
        let mut buffer = [0; 1024];
        match stream.read(&mut buffer) {
            Ok(_) => {
                let response = b"HTTP/1.1 200 OK\r\n\r\nHello, World!";
                stream.write_all(response).unwrap();
            }
            Err(e) => eprintln!("Failed to read from connection: {}", e),
        }
    }
}