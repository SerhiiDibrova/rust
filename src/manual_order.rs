mod manual_order {
    use std::error::Error;
    use std::fmt;
    use chrono::{DateTime, Utc};
    use crate::logger;

    #[derive(Debug)]
    pub struct ManualOrder {
        strategy: i32,
        status: String,
        timestamp: DateTime<Utc>,
    }

    #[derive(Debug)]
    pub struct ManualOrderError {
        details: String,
    }

    impl ManualOrderError {
        pub fn new(msg: &str) -> ManualOrderError {
            ManualOrderError {
                details: msg.to_string(),
            }
        }
    }

    impl fmt::Display for ManualOrderError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.details)
        }
    }

    impl Error for ManualOrderError {}

    impl ManualOrder {
        pub fn new(strategy: i32) -> Result<ManualOrder, ManualOrderError> {
            if strategy < 0 {
                return Err(ManualOrderError::new("Strategy must be a non-negative integer."));
            }
            Ok(ManualOrder {
                strategy,
                status: String::from("Pending"),
                timestamp: Utc::now(),
            })
        }

        pub fn order_event(&self, unique_id: i32) -> Result<(), ManualOrderError> {
            if unique_id < 0 {
                let error_message = format!("ERROR: Order event occurred in method 'order_event' with uniqueId: {}.", unique_id);
                if let Err(e) = logger::log_error(&error_message) {
                    return Err(ManualOrderError::new(&format!("Logging error: {}", e)));
                }
            } else if unique_id > i32::MAX {
                let error_message = format!("ERROR: Order event occurred in method 'order_event' with uniqueId: {}.", unique_id);
                if let Err(e) = logger::log_error(&error_message) {
                    return Err(ManualOrderError::new(&format!("Logging error: {}", e)));
                }
            }
            Ok(())
        }
    }
}