mod error {
    use std::fmt;

    #[derive(Debug)]
    pub enum AppError {
        NotFound(String),
        InvalidInput(String),
        Unauthorized(String),
        InternalError(String),
    }

    impl fmt::Display for AppError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                AppError::NotFound(ref msg) => write!(f, "Not Found: {}", msg),
                AppError::InvalidInput(ref msg) => write!(f, "Invalid Input: {}", msg),
                AppError::Unauthorized(ref msg) => write!(f, "Unauthorized: {}", msg),
                AppError::InternalError(ref msg) => write!(f, "Internal Error: {}", msg),
            }
        }
    }

    impl std::error::Error for AppError {}
}