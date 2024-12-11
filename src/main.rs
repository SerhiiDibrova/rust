package main

use actix_web::{web, App, HttpServer, Responder};
use log::{info, error};
use std::sync::Arc;

struct StrategyHandler;

impl StrategyHandler {
    fn new() -> Self {
        StrategyHandler
    }

    async fn get_stock_packet(&self) -> impl Responder {
        info!("get_stock_packet called");
        match self.fetch_stock_packet().await {
            Ok(data) => data,
            Err(e) => {
                error!("Error fetching stock packet: {}", e);
                "Error fetching stock packet"
            }
        }
    }

    async fn register_for_data(&self) -> impl Responder {
        info!("register_for_data called");
        match self.perform_registration().await {
            Ok(_) => "Registration successful",
            Err(e) => {
                error!("Error during registration: {}", e);
                "Error during registration"
            }
        }
    }

    async fn set_activated_handler(&self) -> impl Responder {
        info!("set_activated_handler called");
        match self.update_activation_status().await {
            Ok(_) => "Activation status updated",
            Err(e) => {
                error!("Error updating activation status: {}", e);
                "Error updating activation status"
            }
        }
    }

    async fn get_strategy_address(&self) -> impl Responder {
        info!("get_strategy_address called");
        match self.fetch_strategy_address().await {
            Ok(address) => address,
            Err(e) => {
                error!("Error fetching strategy address: {}", e);
                "Error fetching strategy address"
            }
        }
    }

    async fn fetch_stock_packet(&self) -> Result<&'static str, &'static str> {
        Ok("Stock packet data")
    }

    async fn perform_registration(&self) -> Result<(), &'static str> {
        Ok(())
    }

    async fn update_activation_status(&self) -> Result<(), &'static str> {
        Ok(())
    }

    async fn fetch_strategy_address(&self) -> Result<&'static str, &'static str> {
        Ok("Strategy address data")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let strategy_handler = Arc::new(StrategyHandler::new());

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::from(strategy_handler.clone()))
            .route("/get_stock_packet", web::get().to(StrategyHandler::get_stock_packet))
            .route("/register_for_data", web::post().to(StrategyHandler::register_for_data))
            .route("/set_activated", web::post().to(StrategyHandler::set_activated_handler))
            .route("/strategy/address", web::get().to(StrategyHandler::get_strategy_address))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}