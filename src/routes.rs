package routes;

use warp::Filter;
use serde_json::json;
use serde::{Deserialize, Serialize};
use log::{info, error};

#[derive(Deserialize)]
struct StrategyDetails {
    id: String,
}

#[derive(Deserialize)]
struct StockPacket {
    field1: String,
    field2: i32,
}

#[derive(Serialize)]
struct ActivationResponse {
    activated: bool,
}

pub struct Strategy {
    active: bool,
}

impl Strategy {
    pub fn new(active: bool) -> Self {
        Strategy { active }
    }

    pub fn register_self(&self) -> Result<(), String> {
        Ok(())
    }

    pub fn activated(&self) -> bool {
        self.active
    }

    pub fn update_arthur(&self, _packet: StockPacket) {
    }
}

pub fn routes() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::post()
        .and(warp::path("register_strategy"))
        .and(warp::body::json())
        .and_then(register_strategy)
        .or(warp::get()
            .and(warp::path("strategy")
            .and(warp::path("activated")))
            .and_then(get_strategy_activation))
        .or(warp::post()
            .and(warp::path("update_arthur"))
            .and(warp::body::json())
            .and_then(update_arthur))
}

async fn register_strategy(details: StrategyDetails) -> Result<impl warp::Reply, warp::Rejection> {
    let strategy = Strategy::new(false);
    match strategy.register_self() {
        Ok(_) => {
            info!("Strategy registered: {}", details.id);
            Ok(warp::reply::json(&json!({"status": "success"})))
        },
        Err(e) => {
            error!("Registration failed: {}", e);
            Err(warp::reject::custom(e))
        }
    }
}

async fn get_strategy_activation() -> Result<impl warp::Reply, warp::Rejection> {
    let strategy = Strategy::new(true);
    let is_active = strategy.activated();
    Ok(warp::reply::json(&ActivationResponse { activated: is_active }))
}

async fn update_arthur(packet: StockPacket) -> Result<impl warp::Reply, warp::Rejection> {
    let strategy = Strategy::new(false);
    strategy.update_arthur(packet);
    Ok(warp::reply::json(&json!({"status": "updated"})))
}