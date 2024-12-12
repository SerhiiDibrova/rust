mod handlers {
    use std::fmt;

    pub struct Request {
        pub path: String,
    }

    pub struct Response {
        pub message: String,
    }

    pub trait HttpHandler {
        fn handle_request(&self, request: &Request) -> Response;
    }

    pub struct GetHandler;

    impl GetHandler {
        pub fn new() -> Self {
            GetHandler
        }
    }

    impl HttpHandler for GetHandler {
        fn handle_request(&self, _request: &Request) -> Response {
            println!("Handling GET request: /about");
            Response {
                message: "This is a lightweight web server designed for handling HTTP requests.".to_string(),
            }
        }
    }
}