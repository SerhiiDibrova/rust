package strategy_stop_request;

use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use log::{info, error};

#[derive(Deserialize)]
struct StopRequest {
    address: String,
}

#[derive(Serialize)]
struct StopResponse {
    success: bool,
}

struct StrategyManager {
    // Assume this struct has necessary methods and fields
}

impl StrategyManager {
    fn stop_strategy(&self, address: &str) -> bool {
        let strategy = MerlinShared::_globalStrategyContainer.find(address);
        if strategy.is_none() {
            return false;
        }
        strategy.unwrap().stop_event_manager();
        true
    }
}

async fn handle_strategy_stop_request(
    req: web::Json<StopRequest>,
    strategy_manager: web::Data<Arc<Mutex<StrategyManager>>>,
) -> impl Responder {
    let address = &req.address;
    let manager = strategy_manager.lock().unwrap();
    
    info!("Received stop request for strategy at address: {}", address);
    
    let success = manager.stop_strategy(address);
    
    if success {
        info!("Successfully stopped strategy at address: {}", address);
        HttpResponse::Ok().json(StopResponse { success: true })
    } else {
        error!("Failed to stop strategy at address: {}", address);
        HttpResponse::NotFound().json(StopResponse { success: false })
    }
}