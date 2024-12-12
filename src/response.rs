mod response {
    pub struct Response {
        pub status_code: u16,
        pub body: String,
    }

    impl Response {
        pub fn new(status_code: u16, body: &str) -> Self {
            Response {
                status_code,
                body: body.to_string(),
            }
        }
    }
}