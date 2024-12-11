package handlers

use hello::{Request, Response};
use crate::strategy::Strategy;

pub fn get_strategy_address(req: &Request) -> Response {
    let strategy = Strategy::new(12345);
    let address = strategy.get_address();
    Response::new()
        .with_status(200)
        .with_body(format!("{{\"address\": \"{}\"}}", address))
}