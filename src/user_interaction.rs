mod user_interaction {
    use std::collections::HashMap;

    pub fn process_user_requests(requests: Vec<UserRequest>) {
        for request in requests {
            match request {
                UserRequest::Login(username, password) => handle_login(username, password),
                UserRequest::Order(order) => handle_order(order),
                UserRequest::Subscription(email) => handle_subscription(email),
            }
        }
    }

    fn handle_login(username: String, password: String) {
        // Logic for handling user login
    }

    fn handle_order(order: Order) {
        // Logic for handling user order
    }

    fn handle_subscription(email: String) {
        // Logic for handling user subscription
    }

    pub enum UserRequest {
        Login(String, String),
        Order(Order),
        Subscription(String),
    }

    pub struct Order {
        pub item_id: u32,
        pub quantity: u32,
    }
}