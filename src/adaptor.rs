package src;

use log::{error, info};
use Lancelot::Exchange;

struct Adaptor;

impl Adaptor {
    pub fn on_disconnection(exchange_: &Exchange) {
        if exchange_.is_valid() {
            match exchange_.to_string() {
                Ok(ref exchange_str) if exchange_str == "NYSE" => {
                    error!("Exchange got disconnected : NYSE");
                }
                Ok(ref exchange_str) if !exchange_str.is_empty() => {
                    error!("Exchange got disconnected : {}", exchange_str);
                }
                Ok(_) => {
                    error!("Exchange got disconnected : ");
                }
                Err(_) => {
                    error!("Exchange got disconnected : Failed to convert exchange to string");
                }
            }
        } else {
            error!("Exchange got disconnected : Invalid Exchange instance");
        }
    }

    pub fn on_connection(exchange_: &Exchange) {
        match exchange_.to_string() {
            Ok(ref exchange_str) if !exchange_str.is_empty() => {
                info!("Exchange connected : {}", exchange_str);
            }
            _ => {
                info!("Exchange connected : ");
            }
        }
    }
}