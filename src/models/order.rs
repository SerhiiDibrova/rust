mod models {
    pub struct DeleteOrderRequest {
        pub params: OrderParams,
    }

    pub struct OrderParams {
        pub order_id: i64,
        pub unique_id: i32,
    }
}