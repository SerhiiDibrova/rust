use actix_web::{web::Json, HttpRequest, HttpResponse};
use serde::Deserialize;
use crate::utils::decrypt::decrypt_message;
use crate::utils::logger::log_cancellation;
use crate::Lancelot;
use std::collections::HashMap;

#[derive(Deserialize)]
struct DeleteOrderRequest {
    order_id: String,
    unique_id: String,
}

pub async fn delete_order(req: HttpRequest) -> HttpResponse {
    let delete_order_request: Result<DeleteOrderRequest, _> = req.json().await;
    if let Err(_) = delete_order_request {
        log_cancellation("Invalid JSON format", "");
        return HttpResponse::BadRequest().body("Invalid JSON format");
    }

    let request = delete_order_request.unwrap();
    let decrypted_data = decrypt_message(&request.order_id);
    let decrypted_data = match decrypted_data {
        Ok(data) => data,
        Err(e) => {
            log_cancellation(&format!("Decryption failed: {}", e), &request.unique_id);
            return HttpResponse::BadRequest().body("Decryption failed");
        }
    };

    let order_id = match decrypted_data.get("order_id") {
        Some(id) => id.clone(),
        None => {
            log_cancellation("Missing order_id in decrypted data", &request.unique_id);
            return HttpResponse::BadRequest().body("Missing order_id in decrypted data");
        }
    };

    let unique_id = match decrypted_data.get("unique_id") {
        Some(id) => id.clone(),
        None => {
            log_cancellation("Missing unique_id in decrypted data", &request.unique_id);
            return HttpResponse::BadRequest().body("Missing unique_id in decrypted data");
        }
    };

    log_cancellation(&order_id, &unique_id);

    if let Some(handler) = Lancelot::get_manual_order_handler() {
        if let Err(e) = handler.place_cancellation_order(unique_id.clone(), 0, 0, Lancelot::API::OrderRequest_CANCEL) {
            log_cancellation(&format!("Error placing cancellation order: {}", e), &unique_id);
            return HttpResponse::InternalServerError().body(format!("Error placing cancellation order: {}", e));
        }
        return HttpResponse::Ok().json(format!("Order cancellation successful for order_id: {}", order_id));
    }
    log_cancellation("Manual order handler not available", &unique_id);
    HttpResponse::InternalServerError().body("Manual order handler not available")
}