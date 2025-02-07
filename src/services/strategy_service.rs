use crate::models::strategy::{Strategy, StrategyParameters};
use crate::errors::service_errors::ServiceError;
use crate::utils::logger::{log_update, log_error};

pub fn update_strategy_parameters(strategy_id: &str, new_parameters: StrategyParameters) -> Result<(), ServiceError> {
    if !strategy_exists(strategy_id) {
        log_error(format!("Strategy with ID {} not found", strategy_id));
        return Err(ServiceError::StrategyNotFound);
    }

    let mut current_strategy = get_strategy(strategy_id).ok_or(ServiceError::StrategyNotFound)?;

    validate_parameters(&new_parameters)?;
    current_strategy.parameters = new_parameters;

    if !save_strategy(&current_strategy) {
        log_error("Failed to save strategy after update".to_string());
        return Err(ServiceError::PersistenceError);
    }

    log_update(strategy_id, &current_strategy.parameters);

    Ok(())
}