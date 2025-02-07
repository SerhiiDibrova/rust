use actix_web::{post, web, HttpResponse, HttpServer, App};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Deserialize)]
struct TradingData {
    symbol: String,
    price: f64,
    volume: f64,
}

#[derive(Serialize)]
struct SerializedResponse {
    data: String,
}

fn serialize_trading_information(data: &TradingData) -> String {
    json!(data).to_string()
}

#[post("/serialize")]
async fn serialize(data: web::Json<TradingData>) -> HttpResponse {
    let serialized_data = serialize_trading_information(&data);
    HttpResponse::Ok().json(SerializedResponse { data: serialized_data })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(serialize)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}