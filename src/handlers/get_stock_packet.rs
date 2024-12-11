package handlers;

use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use std::num::ParseIntError;
use log::{info, error};

#[derive(Deserialize)]
struct QueryParams {
    token: String,
    side: String,
    client: String,
    algo: String,
    ioc: String,
    needEvent: Option<bool>,
}

#[derive(Debug)]
enum Side {
    Buy,
    Sell,
}

impl Side {
    fn from_str(s: &str) -> Result<Self, &'static str> {
        match s {
            "buy" => Ok(Side::Buy),
            "sell" => Ok(Side::Sell),
            _ => Err("Invalid side"),
        }
    }
}

async fn event_registration(client: &str, algo: &str) {
    info!("Registering event for client: {}, algo: {}", client, algo);
}

async fn register_stock_packet(token: u32, side: Side, client: &str, algo: &str, ioc: &str) -> Result<String, &'static str> {
    info!("Registering stock packet with token: {}, side: {:?}, client: {}, algo: {}, ioc: {}", token, side, client, algo, ioc);
    Ok("Stock packet data".to_string())
}

async fn get_stock_packet_handler(query: web::Query<QueryParams>) -> impl Responder {
    let token: Result<u32, ParseIntError> = query.token.parse();
    if let Err(_) = token {
        error!("Invalid token: {}", query.token);
        return HttpResponse::BadRequest().body("Token must be a valid unsigned integer");
    }

    let side = Side::from_str(&query.side);
    if let Err(_) = side {
        error!("Invalid side: {}", query.side);
        return HttpResponse::BadRequest().body("Side must be either 'buy' or 'sell'");
    }

    if query.client.is_empty() || query.algo.is_empty() {
        error!("Client or algo is empty");
        return HttpResponse::BadRequest().body("Client and algo cannot be empty");
    }

    let valid_ioc_values = vec!["GTC", "IOC", "FOK"];
    if !valid_ioc_values.contains(&query.ioc.as_str()) {
        error!("Invalid ioc value: {}", query.ioc);
        return HttpResponse::BadRequest().body("IOC must be one of the following: GTC, IOC, FOK");
    }

    if query.needEvent.unwrap_or(false) {
        event_registration(&query.client, &query.algo).await;
    }

    match register_stock_packet(token.unwrap(), side.unwrap(), &query.client, &query.algo, &query.ioc).await {
        Ok(stock_packet) => {
            info!("Stock packet retrieved successfully");
            HttpResponse::Ok().json(stock_packet)
        },
        Err(e) => {
            error!("Error retrieving stock packet: {}", e);
            HttpResponse::InternalServerError().body("Failed to retrieve stock packet due to internal error")
        }
    }
}