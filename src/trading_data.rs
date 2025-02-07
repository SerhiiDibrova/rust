mod trading_data {
    use std::option::Option;

    pub struct TradingData {
        trade_id: u32,
        symbol: String,
        quantity: f64,
        price: f64,
        timestamp: String,
        comment: Option<String>,
    }

    impl TradingData {
        pub fn new(trade_id: u32, symbol: String, quantity: f64, price: f64, timestamp: String, comment: Option<String>) -> Self {
            TradingData {
                trade_id,
                symbol,
                quantity,
                price,
                timestamp,
                comment,
            }
        }

        pub fn get_trade_id(&self) -> u32 {
            self.trade_id
        }

        pub fn get_symbol(&self) -> &String {
            &self.symbol
        }

        pub fn get_quantity(&self) -> f64 {
            self.quantity
        }

        pub fn get_price(&self) -> f64 {
            self.price
        }

        pub fn get_timestamp(&self) -> &String {
            &self.timestamp
        }

        pub fn get_comment(&self) -> Option<&String> {
            self.comment.as_ref()
        }

        pub fn has_comment(&self) -> bool {
            self.comment.is_some()
        }
    }
}