package handler;

use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::atomic::{AtomicUsize, Ordering};
use log::{error, info};

static GLOBAL_JSON_ID: AtomicUsize = AtomicUsize::new(0);

const VALID_STRATEGIES: [&str; 3] = ["strategy1", "strategy2", "strategy3"];

#[derive(Deserialize)]
pub struct StrategyQuery {
    pub strategy_: Option<String>,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

pub async fn get_strategy_status(query: web::Query<StrategyQuery>) -> impl Responder {
    let strategy_id = query.strategy_.as_deref().unwrap_or("");

    if strategy_id.is_empty() {
        let error_message = "Invalid strategy_ parameter".to_string();
        error!("{}", error_message);
        return HttpResponse::BadRequest().json(ErrorResponse { error: error_message });
    }

    if !VALID_STRATEGIES.contains(&strategy_id) {
        let error_message = "Strategy not recognized".to_string();
        error!("{}", error_message);
        return HttpResponse::BadRequest().json(ErrorResponse { error: error_message });
    }

    let json_id = GLOBAL_JSON_ID.fetch_add(1, Ordering::SeqCst);
    let response = json!({
        "id": json_id,
        "strategy_": strategy_id,
        "status": "success"
    });

    info!("Successfully retrieved status for strategy: {}", strategy_id);
    HttpResponse::Ok().json(response)
}