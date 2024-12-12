mod order_handler {
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};
    use warp::http::StatusCode;
    use warp::{self, Filter};
    use serde_json::Value;

    #[derive(Debug)]
    struct OrderPacket {
        unique_id: u32,
        price: u32,
        quantity: u32,
    }

    impl OrderPacket {
        fn execute_order(&self, price: u32, quantity: u32) -> Result<(), String> {
            // Logic to execute the order
            Ok(())
        }
    }

    struct OrderHandler {
        manual_order_container: Arc<Mutex<HashMap<u32, OrderPacket>>>,
        next_unique_id: Arc<Mutex<u32>>,
    }

    impl OrderHandler {
        fn new() -> Self {
            OrderHandler {
                manual_order_container: Arc::new(Mutex::new(HashMap::new())),
                next_unique_id: Arc::new(Mutex::new(1)),
            }
        }

        fn place_order(
            &self,
            unique_id: u32,
            price: u32,
            quantity: u32,
        ) -> Result<StatusCode, String> {
            if unique_id == 0 || price == 0 || quantity == 0 {
                return Err("400 Bad Request: uniqueId, price, and quantity must be positive integers.".to_string());
            }

            let container = self.manual_order_container.lock().unwrap();
            match container.get(&unique_id) {
                Some(order_packet) => {
                    order_packet.execute_order(price, quantity).map_err(|e| {
                        log::error!("Error executing order: {}", e);
                        e.to_string()
                    })?;
                    Ok(StatusCode::OK)
                }
                None => {
                    log::warn!("Order packet not found for unique_id: {}", unique_id);
                    Err("404 Not Found: Order packet not found.".to_string())
                }
            }
        }

        fn handle_new_order(
            &self,
            token: u32,
            side: &str,
            client: &str,
            algo: &str,
            ioc: u8,
        ) -> Result<u32, String> {
            if token < 0 {
                return Err("400 Bad Request: token must be a non-negative integer.".to_string());
            }
            if side != "BUY" && side != "SELL" {
                return Err("400 Bad Request: side must be either 'BUY' or 'SELL'.".to_string());
            }
            if client.is_empty() || client.len() > 50 {
                return Err("400 Bad Request: client must be a non-empty string with a maximum length of 50 characters.".to_string());
            }
            if algo.is_empty() || algo.len() > 50 {
                return Err("400 Bad Request: algo must be a non-empty string with a maximum length of 50 characters.".to_string());
            }
            if ioc != 0 && ioc != 1 {
                return Err("400 Bad Request: ioc must be either 0 or 1.".to_string());
            }

            let unique_id = self.generate_unique_id();
            let new_order_packet = OrderPacket {
                unique_id,
                price: 0,
                quantity: 0,
            };

            let mut container = self.manual_order_container.lock().unwrap();
            container.insert(unique_id, new_order_packet);
            Ok(unique_id)
        }

        fn generate_unique_id(&self) -> u32 {
            let mut id = self.next_unique_id.lock().unwrap();
            let unique_id = *id;
            *id += 1;
            unique_id
        }
    }

    pub fn routes(order_handler: OrderHandler) -> impl Filter<Extract = (StatusCode,), Error = warp::Rejection> + Clone {
        warp::post()
            .and(warp::path("api")
            .and(warp::path("orders")))
            .and(warp::body::json())
            .map(move |body: Value| {
                let token = body["token"].as_u64().unwrap() as u32;
                let side = body["side"].as_str().unwrap();
                let client = body["client"].as_str().unwrap();
                let algo = body["algo"].as_str().unwrap();
                let ioc = body["ioc"].as_u64().unwrap() as u8;

                order_handler.handle_new_order(token, side, client, algo, ioc)
                    .map(|unique_id| StatusCode::OK)
                    .unwrap_or_else(|e| StatusCode::BAD_REQUEST)
            })
    }
}