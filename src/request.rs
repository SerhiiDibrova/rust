mod request {
    pub struct Request {
        pub method: String,
        pub path: String,
    }

    impl Request {
        pub fn new(method: &str, path: &str) -> Self {
            Request {
                method: method.to_string(),
                path: path.to_string(),
            }
        }
    }
}