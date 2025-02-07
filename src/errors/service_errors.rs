mod service_errors {
    #[derive(Debug)]
    pub enum ServiceError {
        StrategyNotFound,
        InvalidParameters,
        PersistenceError,
        // Add more error types as needed
    }
}