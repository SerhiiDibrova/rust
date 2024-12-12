mod server {
    use std::net::{TcpListener, TcpStream};
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::io::{Read, Write};
    use log::{info, error};

    pub struct Config {
        pub port: u16,
    }

    pub struct Server {
        port: u16,
        thread_pool: Arc<Mutex<Vec<thread::JoinHandle<()>>>>,
    }

    impl Server {
        pub fn new(port: u16) -> Self {
            Server {
                port,
                thread_pool: Arc::new(Mutex::new(Vec::new())),
            }
        }

        pub fn run(&self, config: &Config) {
            if config.port < 1024 || config.port > 65535 {
                panic!("Port number must be between 1024 and 65535");
            }
            info!("Starting server on port {}", config.port);
            let listener = TcpListener::bind(format!("0.0.0.0:{}", config.port)).unwrap();
            let thread_pool = Arc::clone(&self.thread_pool);

            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => {
                        let thread_pool = Arc::clone(&thread_pool);
                        let handle = thread::spawn(move || {
                            handle_connection(stream);
                        });
                        thread_pool.lock().unwrap().push(handle);
                    }
                    Err(e) => {
                        error!("Error accepting connection: {}", e);
                    }
                }
            }
        }

        pub fn shutdown(&self) {
            let handles = self.thread_pool.lock().unwrap();
            for handle in handles.iter() {
                handle.join().unwrap();
            }
        }
    }

    fn handle_connection(mut stream: TcpStream) {
        let mut buffer = [0; 1024];
        match stream.read(&mut buffer) {
            Ok(size) => {
                if let Err(e) = stream.write_all(&buffer[0..size]) {
                    error!("Failed to write to connection: {}", e);
                }
            }
            Err(e) => {
                error!("Failed to read from connection: {}", e);
            }
        }
    }
}