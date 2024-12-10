package handlers

use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct StockPacket {
    token: String,
    side: String,
    client: String,
    algo: String,
    ioc: bool,
    strategy: String,
}

#[derive(Deserialize)]
struct StockPacketRequest {
    token: String,
    side: String,
    client: String,
    algo: String,
    ioc: bool,
    strategy: String,
}

fn validate_request(req: &StockPacketRequest) -> Result<(), String> {
    if req.token.is_empty() {
        return Err("Token cannot be empty".into());
    }
    if req.side.is_empty() || !["buy", "sell"].contains(&req.side.as_str()) {
        return Err("Side must be 'buy' or 'sell'".into());
    }
    if req.client.is_empty() {
        return Err("Client cannot be empty".into());
    }
    if req.algo.is_empty() || !["algo1", "algo2"].contains(&req.algo.as_str()) {
        return Err("Algo must be a valid algorithm".into());
    }
    if req.strategy.is_empty() {
        return Err("Strategy cannot be empty".into());
    }
    Ok(())
}

pub async fn register_stock_packet(req: web::Json<StockPacketRequest>) -> impl Responder {
    match validate_request(&req) {
        Ok(_) => {
            let stock_packet = StockPacket {
                token: req.token.clone(),
                side: req.side.clone(),
                client: req.client.clone(),
                algo: req.algo.clone(),
                ioc: req.ioc,
                strategy: req.strategy.clone(),
            };
            HttpResponse::Ok().json(stock_packet)
        }
        Err(err) => HttpResponse::BadRequest().body(err),
    }
}