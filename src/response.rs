mod response {
    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize)]
    pub struct Response<T> {
        pub status: String,
        pub message: String,
        pub data: Option<T>,
    }

    impl<T> Response<T> {
        pub fn new(status: String, message: String, data: Option<T>) -> Self {
            Response { status, message, data }
        }

        pub fn to_json(&self) -> String {
            serde_json::to_string(self).unwrap()
        }
    }
}