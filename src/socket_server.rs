mod socket_server {
    use std::net::{TcpListener, TcpStream};
    use std::sync::{Arc, Mutex};
    use tokio::runtime::Runtime;
    use tokio::task;
    use std::io::{self, Read, Write};
    use log::{error, info};

    pub struct SocketServer {
        listener: TcpListener,
        port: u16,
        _io_context: Runtime,
        _error_code: Option<String>,
    }

    impl SocketServer {
        pub fn new(port: u16) -> Result<SocketServer, String> {
            if port < 1 || port > 65535 {
                return Err("Port must be between 1 and 65535".to_string());
            }
            let listener = TcpListener::bind(format!("127.0.0.1:{}", port))
                .map_err(|e| e.to_string())?;
            let runtime = Runtime::new().map_err(|e| e.to_string())?;
            Ok(SocketServer {
                listener,
                port,
                _io_context: runtime,
                _error_code: None,
            })
        }

        pub fn run_server(&self) {
            if let Err(e) = self._io_context.block_on(async {
                self.start_accept().await;
            }) {
                error!("Error running server: {}", e);
            }
        }

        pub async fn start_accept(&self) {
            loop {
                match self.listener.accept() {
                    Ok((stream, _)) => {
                        let stream = Arc::new(Mutex::new(stream));
                        task::spawn(self.handle_connection(stream));
                    }
                    Err(e) => {
                        error!("Error accepting connection: {}", e);
                    }
                }
            }
        }

        async fn handle_connection(stream: Arc<Mutex<TcpStream>>) {
            let mut buffer = [0; 1024];
            let mut stream = stream.lock().unwrap();
            match stream.read(&mut buffer) {
                Ok(size) => {
                    if let Err(e) = stream.write_all(&buffer[..size]) {
                        error!("Error writing to stream: {}", e);
                    }
                }
                Err(e) => {
                    error!("Error reading from stream: {}", e);
                }
            }
        }

        pub fn handle_accept(&self, error: Option<io::Error>) {
            if let Some(err) = error {
                error!("Error in handle_accept: {}", err);
                return;
            }
            self.start_accept();
        }

        pub fn start_listening(&self) {
            self.run_server();
        }
    }
}