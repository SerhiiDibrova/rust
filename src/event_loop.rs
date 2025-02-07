mod event_loop {
    use std::net::{TcpListener, TcpStream};
    use std::io::{Read, Write};
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::sync::mpsc;
    use std::time::Duration;
    use std::signal::Signal;
    use std::signal::SignalKind;

    pub struct EventLoop {
        listener: TcpListener,
        shutdown_signal: Arc<Mutex<bool>>,
    }

    impl EventLoop {
        pub fn new(address: &str) -> std::io::Result<Self> {
            let listener = TcpListener::bind(address)?;
            let shutdown_signal = Arc::new(Mutex::new(false));
            Ok(EventLoop { listener, shutdown_signal })
        }

        pub fn run_event_loop(&self) {
            let shutdown_signal = Arc::clone(&self.shutdown_signal);
            let (tx, rx) = mpsc::channel();

            thread::spawn(move || {
                for stream in self.listener.incoming() {
                    match stream {
                        Ok(stream) => {
                            let tx = tx.clone();
                            thread::spawn(move || {
                                handle_connection(stream);
                                tx.send(()).unwrap();
                            });
                        }
                        Err(e) => eprintln!("Error accepting connection: {}", e),
                    }
                }
            });

            let shutdown_signal_clone = Arc::clone(&shutdown_signal);
            thread::spawn(move || {
                let signal = Signal::new(SignalKind::interrupt()).unwrap();
                signal.recv().unwrap();
                let mut shutdown = shutdown_signal_clone.lock().unwrap();
                *shutdown = true;
            });

            while !*shutdown_signal.lock().unwrap() {
                if let Ok(_) = rx.recv_timeout(Duration::from_millis(100)) {
                    continue;
                }
            }
        }
    }

    fn handle_connection(mut stream: TcpStream) {
        let mut buffer = [0; 1024];
        while match stream.read(&mut buffer) {
            Ok(size) if size > 0 => {
                stream.write_all(&buffer[0..size]).is_ok()
            }
            _ => false,
        } {}
    }
}