mod models {
    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize)]
    pub struct OrderRequest {
        pub order_type: String,
        pub asset_details: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct OrderPacket {
        pub order_request: OrderRequest,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Order {
        pub token: String,
        pub side: String,
        pub client: String,
        pub algo: String,
        pub ioc: bool,
    }

    impl OrderPacket {
        pub fn execute_order(&self, order: &Order) {
            match self.order_request.order_type.as_str() {
                "buy" => {
                    println!("Executing buy order for token: {}", order.token);
                }
                "sell" => {
                    println!("Executing sell order for token: {}", order.token);
                }
                _ => {
                    println!("Invalid order type: {}", self.order_request.order_type);
                }
            }
        }
    }
}