use std::collections::HashMap;
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use log::{info, error};
use crate::config::Config;

struct Connection {
    stream: TcpStream,
    login_id: String,
}

struct Server {
    listener: TcpListener,
    connections: Mutex<HashMap<String, Arc<Connection>>>,
    port: u16,
}

impl Server {
    fn load() -> Result<(), String> {
        info!("Starting the loading process.");
        let config = load_configuration()?;
        let server = Server::initialize_server(config.port)?;
        server.initialize_database_connection();
        Ok(())
    }

    fn initialize_server(port: u16) -> Result<Self, String> {
        if port < 1 || port > 65535 {
            return Err("Port number must be between 1 and 65535.".to_string());
        }
        let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).map_err(|e| e.to_string())?;
        info!("Server bound to port {}", port);
        Ok(Server {
            listener,
            connections: Mutex::new(HashMap::new()),
            port,
        })
    }

    fn accept_connection(&self) {
        info!("Ready to accept connections.");
        match self.listener.accept() {
            Ok((stream, addr)) => {
                info!("Accepted connection from {}", addr);
                let connection = Arc::new(Connection {
                    stream,
                    login_id: String::new(), // Placeholder for login ID
                });
                self.connections.lock().unwrap().insert(connection.login_id.clone(), connection);
            }
            Err(e) => {
                error!("Failed to accept connection: {}", e);
            }
        }
    }

    fn remove_connection(&self, login_id: &str) {
        let mut connections = self.connections.lock().unwrap();
        if connections.remove(login_id).is_some() {
            info!("Removed connection for login ID: {}", login_id);
        } else {
            info!("No connection found for login ID: {}", login_id);
        }
    }

    fn initialize_database_connection(&self) {
        // Database connection logic goes here
    }
}

fn load_configuration() -> Result<Config, String> {
    // Configuration loading logic goes here
    Ok(Config::default())
}