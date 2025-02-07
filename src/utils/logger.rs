use std::fmt::Debug;

pub struct StrategyParameters {
    // Define fields as needed
}

pub fn log_update(strategy_id: &str, parameters: &StrategyParameters) {
    println!("Updated strategy {}: {:?}", strategy_id, parameters);
}

pub fn log_error(message: String) {
    eprintln!("Error: {}", message);
}