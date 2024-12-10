package handlers;

use actix_web::{web, HttpResponse, Responder, HttpRequest};
use log::{info, warn, error};
use serde_json::json;
use crate::global_connection_container::GlobalConnectionContainer;

pub async fn handle_get_connection_request(req: HttpRequest) -> impl Responder {
    let login_id = req.query().get("loginId");
    if login_id.is_none() || login_id.unwrap().is_empty() {
        return HttpResponse::BadRequest().finish();
    }
    
    let login_id = login_id.unwrap();
    info!("Received request to retrieve connection for loginId: {}", login_id);

    match GlobalConnectionContainer::get_connection(login_id).await {
        Ok(connection) => {
            if connection.is_empty() {
                warn!("Connection is empty for loginId: {}", login_id);
                return HttpResponse::NotFound().finish();
            }
            HttpResponse::Ok().json(connection)
        },
        Err(err) => {
            error!("Error retrieving connection for loginId: {}. Error: {}", login_id, err);
            HttpResponse::InternalServerError().finish()
        }
    }
}