package adaptors;

use std::f64;

pub struct Adaptor;

impl Adaptor {
    pub fn new() -> Self {
        Adaptor
    }

    pub fn strategy_a(&self, market_data: &MarketData) -> Result<Trade, String> {
        let signal = self.analyze_market(market_data)?;
        match signal {
            Signal::Buy => Ok(Trade::new("Buy", market_data.price)),
            Signal::Sell => Ok(Trade::new("Sell", market_data.price)),
        }
    }

    pub fn strategy_b(&self, market_data: &MarketData) -> Result<Trade, String> {
        let trend = self.detect_trend(market_data)?;
        match trend {
            Trend::Upward => Ok(Trade::new("Buy", market_data.price)),
            Trend::Downward => Ok(Trade::new("Sell", market_data.price)),
        }
    }

    pub fn strategy_c(&self, market_data: &MarketData) -> Result<Trade, String> {
        let volatility = self.calculate_volatility(market_data)?;
        if volatility > THRESHOLD {
            Ok(Trade::new("Buy", market_data.price))
        } else {
            Ok(Trade::new("Sell", market_data.price))
        }
    }

    fn analyze_market(&self, market_data: &MarketData) -> Result<Signal, String> {
        if market_data.price > 0.0 {
            Ok(Signal::Buy)
        } else {
            Err("Invalid market data".to_string())
        }
    }

    fn detect_trend(&self, market_data: &MarketData) -> Result<Trend, String> {
        if market_data.price > 0.0 {
            Ok(Trend::Upward)
        } else {
            Err("Invalid market data".to_string())
        }
    }

    fn calculate_volatility(&self, market_data: &MarketData) -> Result<f64, String> {
        if market_data.price > 0.0 {
            Ok(0.1)
        } else {
            Err("Invalid market data".to_string())
        }
    }
}

pub struct MarketData {
    pub price: f64,
}

pub struct Trade {
    pub action: String,
    pub price: f64,
}

impl Trade {
    pub fn new(action: &str, price: f64) -> Self {
        Trade {
            action: action.to_string(),
            price,
        }
    }
}

pub enum Signal {
    Buy,
    Sell,
}

pub enum Trend {
    Upward,
    Downward,
}

const THRESHOLD: f64 = 0.05;