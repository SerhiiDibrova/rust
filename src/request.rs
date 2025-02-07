mod request {
    use std::collections::HashMap;

    #[derive(Debug)]
    pub enum RequestType {
        LOGIN,
        ORDER,
        SUBSCRIBE,
    }

    #[derive(Debug)]
    pub struct Request {
        pub request_type: RequestType,
        pub data: HashMap<String, String>,
    }

    impl Request {
        pub fn new(request_type: RequestType, data: HashMap<String, String>) -> Self {
            Request { request_type, data }
        }

        pub fn validate(&self) -> bool {
            match self.request_type {
                RequestType::LOGIN => self.data.contains_key("username") && self.data.contains_key("password"),
                RequestType::ORDER => self.data.contains_key("order_id"),
                RequestType::SUBSCRIBE => self.data.contains_key("email"),
            }
        }

        pub fn extract_data(&self) -> &HashMap<String, String> {
            &self.data
        }
    }
}