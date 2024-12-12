mod error {
    use std::fmt;

    #[derive(Debug)]
    pub enum OrderError {
        InvalidInput(String),
        PacketCreationFailed(String),
        StorageFailed(String),
    }

    impl fmt::Display for OrderError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                OrderError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
                OrderError::PacketCreationFailed(msg) => write!(f, "Packet creation failed: {}", msg),
                OrderError::StorageFailed(msg) => write!(f, "Storage operation failed: {}", msg),
            }
        }
    }

    impl std::error::Error for OrderError {}

    pub fn error_message(err: &OrderError) -> String {
        format!("{}", err)
    }

    pub fn handle_order_error(err: OrderError) -> String {
        error_message(&err)
    }
}