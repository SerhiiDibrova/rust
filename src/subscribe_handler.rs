package src;

use actix_web::{web, HttpResponse, Responder};
use serde_json::Value;
use log::{error, info};
use std::sync::Arc;

struct Strategy {
    id: String,
    status: String,
}

impl Strategy {
    fn load(strategy_id: &str) -> Result<Strategy, String> {
        if strategy_id.is_empty() {
            return Err("Strategy ID cannot be empty".to_string());
        }
        // Simulate loading strategy, return error if not found
        if strategy_id != "valid_strategy_id" {
            return Err("Strategy not found".to_string());
        }
        Ok(Strategy {
            id: strategy_id.to_string(),
            status: "active".to_string(),
        })
    }

    fn encrypt_status(&self) -> String {
        format!("encrypted_{}", self.status)
    }
}

fn demangle_user_id(user_id: &str) -> String {
    user_id.to_string() // Implement actual demangling logic here
}

async fn subscribe(req_body: web::Json<Value>) -> impl Responder {
    let strategy_id = req_body.get("strategy_").and_then(Value::as_str).unwrap_or("");
    let name = req_body.get("name_").and_then(Value::as_str).unwrap_or("");
    let param = req_body.get("param_").and_then(Value::as_str).unwrap_or("");

    if strategy_id.is_empty() || name.is_empty() || param.is_empty() {
        error!("Missing required parameters");
        return HttpResponse::BadRequest().json(json!({"error": "Missing required parameters"}));
    }

    let user_id = demangle_user_id("demangled_user_id");

    match Strategy::load(strategy_id) {
        Ok(strategy) => {
            if strategy.status != "active" {
                error!("Strategy is inactive");
                return HttpResponse::BadRequest().json(json!({"error": "Strategy is inactive"}));
            }
            let encrypted_status = strategy.encrypt_status();
            let response = json!({
                "success": true,
                "user_id": user_id,
                "strategy_id": strategy.id,
                "encrypted_status": encrypted_status,
                "name": name,
                "param": param,
            });
            info!("Successfully processed subscription for user_id: {}", user_id);
            HttpResponse::Ok().json(response)
        }
        Err(err) => {
            error!("Error loading strategy: {}", err);
            HttpResponse::NotFound().json(json!({"error": "Strategy not found"}))
        }
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/subscribe").route(web::post().to(subscribe)));
}