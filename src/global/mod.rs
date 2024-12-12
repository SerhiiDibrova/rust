mod arthur_message_manager;
mod message_queue;
mod connection_manager;
mod serialization;
mod encryption;
mod logging;

pub use arthur_message_manager::ArthurMessageManagerThread;

pub struct Global {
    message_manager: arthur_message_manager::ArthurMessageManager,
    queue: message_queue::MessageQueue,
    connection: connection_manager::ConnectionManager,
    serializer: serialization::Serializer,
    encryptor: encryption::Encryptor,
    logger: logging::Logger,
    running: bool,
}

impl Global {
    pub fn new() -> Self {
        Global {
            message_manager: arthur_message_manager::ArthurMessageManager::new(),
            queue: message_queue::MessageQueue::new(),
            connection: connection_manager::ConnectionManager::new(),
            serializer: serialization::Serializer::new(),
            encryptor: encryption::Encryptor::new(),
            logger: logging::Logger::new(),
            running: false,
        }
    }

    pub fn run(&mut self) {
        self.logger.log("Global module starting...");
        self.running = true;

        if let Err(e) = self.message_manager.start() {
            self.logger.log(&format!("Failed to start message manager: {}", e));
            return;
        }

        if let Err(e) = self.connection.establish() {
            self.logger.log(&format!("Failed to establish connection: {}", e));
            return;
        }

        while self.running {
            match self.queue.get_message() {
                Ok(message) => {
                    if let Err(e) = self.validate_message(&message) {
                        self.logger.log(&format!("Invalid message format: {}", e));
                        continue;
                    }
                    match self.serializer.serialize(&message) {
                        Ok(serialized_message) => {
                            match self.encryptor.encrypt(&serialized_message) {
                                Ok(encrypted_message) => {
                                    self.logger.log("Message processed and encrypted.");
                                }
                                Err(e) => {
                                    self.logger.log(&format!("Encryption failed: {}", e));
                                }
                            }
                        }
                        Err(e) => {
                            self.logger.log(&format!("Serialization failed: {}", e));
                        }
                    }
                }
                Err(e) => {
                    self.logger.log(&format!("Failed to get message: {}", e));
                }
            }
        }
    }

    pub fn stop(&mut self) {
        self.logger.log("Stopping Global module...");
        if self.running {
            self.running = false;
            if let Err(e) = self.message_manager.stop() {
                self.logger.log(&format!("Failed to stop message manager: {}", e));
            }
            if let Err(e) = self.connection.close() {
                self.logger.log(&format!("Failed to close connection: {}", e));
            }
        }
    }

    fn validate_message(&self, message: &str) -> Result<(), String> {
        if message.is_empty() {
            return Err("Message cannot be empty".to_string());
        }
        // Additional validation logic can be added here
        Ok(())
    }
}