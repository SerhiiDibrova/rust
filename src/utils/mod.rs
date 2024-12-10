package utils;

use serde_json::Value;
use log::{info, error, debug, warn};
use actix_web::{HttpResponse, HttpRequest};

pub fn parse_json(data: &str) -> Result<Value, serde_json::Error> {
    serde_json::from_str(data)
}

pub fn log(message: &str, level: &str) {
    match level {
        "info" => info!("{}", message),
        "debug" => debug!("{}", message),
        "warn" => warn!("{}", message),
        "error" => error!("{}", message),
        _ => info!("{}", message),
    }
}

pub fn log_request(req: &HttpRequest) {
    let method = req.method();
    let path = req.path();
    log(&format!("Received request: {} {}", method, path), "info");
}

pub fn handle_error(err: &str, error_type: &str) -> HttpResponse {
    match error_type {
        "not_found" => {
            error!("{}", err);
            HttpResponse::NotFound().body(err)
        },
        "bad_request" => {
            error!("{}", err);
            HttpResponse::BadRequest().body(err)
        },
        "unauthorized" => {
            error!("{}", err);
            HttpResponse::Unauthorized().body(err)
        },
        "forbidden" => {
            error!("{}", err);
            HttpResponse::Forbidden().body(err)
        },
        "conflict" => {
            error!("{}", err);
            HttpResponse::Conflict().body(err)
        },
        _ => {
            error!("{}", err);
            HttpResponse::InternalServerError().body(err)
        },
    }
}