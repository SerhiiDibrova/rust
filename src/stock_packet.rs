package stock_packet

struct StockPacket {
    strategy_number: u32,
    unique_id: String,
    token: String,
    quantity: u32,
    filled_quantity: u32,
    order_id: String,
    price: f64,
    last_trade_price: f64,
    side: String,
    client_code: String,
    contract_description: String,
}

impl StockPacket {
    pub fn new(strategy_number: u32, unique_id: String, token: String, quantity: u32, filled_quantity: u32, order_id: String, price: f64, last_trade_price: f64, side: String, client_code: String, contract_description: String) -> Self {
        Self {
            strategy_number,
            unique_id,
            token,
            quantity,
            filled_quantity,
            order_id,
            price,
            last_trade_price,
            side,
            client_code,
            contract_description,
        }
    }

    pub fn get_strategy_number(&self) -> u32 {
        self.strategy_number
    }

    pub fn get_unique_id(&self) -> &String {
        &self.unique_id
    }

    pub fn get_token(&self) -> &String {
        &self.token
    }

    pub fn get_quantity(&self) -> u32 {
        self.quantity
    }

    pub fn get_filled_quantity(&self) -> u32 {
        self.filled_quantity
    }

    pub fn get_order_id(&self) -> &String {
        &self.order_id
    }

    pub fn get_price(&self) -> f64 {
        self.price
    }

    pub fn get_last_trade_price(&self) -> f64 {
        self.last_trade_price
    }

    pub fn get_side(&self) -> &String {
        &self.side
    }

    pub fn get_client_code(&self) -> &String {
        &self.client_code
    }

    pub fn get_contract_description(&self) -> &String {
        &self.contract_description
    }
}