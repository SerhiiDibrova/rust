package order_handler;

use serde_json::{Value, json};
use std::io::{BufReader, Read, Write};
use std::net::TcpStream;
use log::{info, error};
use crate::utils::{decrypt_request, validate_order_parameters, place_order, modify_order_logic};

const JSON_ID: &str = "id";
const JSON_TOKEN: &str = "token";
const JSON_PRICE: &str = "price";
const JSON_QUANTITY: &str = "quantity";
const JSON_CLIENT: &str = "client";
const JSON_SIDE: &str = "side";
const JSON_ORDER_TYPE: &str = "order_type";

pub fn handle_request(stream: &mut TcpStream) {
    let mut reader = BufReader::new(stream);
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer).unwrap();
    
    let decrypted_message = decrypt_request(&buffer);
    let json: Value = serde_json::from_str(&decrypted_message).unwrap();
    
    let missing_params: Vec<&str> = vec![
        JSON_ID, JSON_TOKEN, JSON_PRICE, JSON_QUANTITY, JSON_CLIENT, JSON_SIDE, JSON_ORDER_TYPE
    ].into_iter()
    .filter(|&param| !json[param].is_string() || json[param].as_str().unwrap().is_empty())
    .collect();
    
    if missing_params.is_empty() && validate_order_parameters(&json) {
        let price: f64 = json[JSON_PRICE].as_str().unwrap().parse().unwrap_or(0.0);
        let quantity: i32 = json[JSON_QUANTITY].as_i64().unwrap_or(0) as i32;
        
        if price > 0.0 && quantity > 0 {
            place_order(&json[JSON_CLIENT].as_str().unwrap(), price, quantity, 
                        json[JSON_SIDE].as_str().unwrap(), json[JSON_ORDER_TYPE].as_str().unwrap());
            
            let response = json!({
                "status": "success",
                "message": "Order placed successfully",
                "id": json[JSON_ID],
                "client": json[JSON_CLIENT],
                "price": price,
                "quantity": quantity,
                "side": json[JSON_SIDE],
                "order_type": json[JSON_ORDER_TYPE]
            });
            
            let response_string = serde_json::to_string(&response).unwrap();
            stream.write_all(response_string.as_bytes()).unwrap();
        } else {
            let response = json!({
                "status": "error",
                "message": "Invalid price or quantity"
            });
            
            let response_string = serde_json::to_string(&response).unwrap();
            stream.write_all(response_string.as_bytes()).unwrap();
        }
    } else {
        let response = json!({
            "status": "error",
            "message": "Invalid parameters",
            "missing": missing_params
        });
        
        let response_string = serde_json::to_string(&response).unwrap();
        stream.write_all(response_string.as_bytes()).unwrap();
    }
}

pub fn modify_order(request_data: &str) {
    let decrypted_message = decrypt_request(request_data);
    let json: Value = serde_json::from_str(&decrypted_message).unwrap();
    
    let price: f64 = json[JSON_PRICE].as_str().unwrap().parse().unwrap_or(0.0);
    let quantity: i32 = json[JSON_QUANTITY].as_i64().unwrap_or(0) as i32;
    let unique_id: &str = json[JSON_ID].as_str().unwrap();
    
    if price > 0.0 && quantity > 0 && modify_order_logic(unique_id, price, quantity) {
        info!("Order modified successfully for ID: {}", unique_id);
        let response = json!({
            "status": "success",
            "message": "Order modified successfully",
            "id": unique_id,
            "price": price,
            "quantity": quantity
        });
        let response_string = serde_json::to_string(&response).unwrap();
        info!("Response: {}", response_string);
    } else {
        error!("Failed to modify order for ID: {}", unique_id);
        let response = json!({
            "status": "error",
            "message": "Failed to modify order"
        });
        let response_string = serde_json::to_string(&response).unwrap();
        info!("Response: {}", response_string);
    }
}