package models;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Communication {
    user_id: String,
    request_type: String,
    additional_data: String,
}

#[derive(Serialize, Deserialize)]
pub struct ManualOrder {
    order_id: String,
    user_id: String,
    product_id: String,
    quantity: i32,
    price: f64,
    status: String,
}

#[derive(Serialize, Deserialize)]
pub struct NewOrderRequest {
    user_id: String,
    product_id: String,
    quantity: i32,
    price: f64,
}

#[derive(Serialize, Deserialize)]
pub struct NewOrderResponse {
    order_id: String,
    status: String,
    message: String,
}

#[derive(Serialize, Deserialize)]
pub struct CancelOrderRequest {
    json_id: i32,
    json_params: CancelOrderParams,
}

#[derive(Serialize, Deserialize)]
pub struct CancelOrderParams {
    json_order_id: i64,
    json_unique_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct SubscriptionRequest {
    strategy_: String,
    name_: String,
    param_: String,
}

#[derive(Serialize, Deserialize)]
pub struct SubscriptionResponse {
    response_status: String,
}