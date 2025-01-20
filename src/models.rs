use std::string::ToString;
use std::time::SystemTime;
use crate::errors::AppErrors;

pub struct Money {
    amount: f64
}

impl Money {
    pub fn new(amount: f64) -> Result<Self, AppErrors> {
        let amount_str = amount.to_string();
        let digits_count = if let Some(pos) = amount_str.find('.') {
            amount_str[pos + 1..].trim_end_matches('0').len()
        } else {
            0
        };
        if amount < 0.0 {
            Err(AppErrors::ModelCreationError(format!("The money amount can't be below 0. Amount provided: {amount}")))
        } else if digits_count > 2 {
            Err(AppErrors::ModelCreationError(format!("There were too many digits. Amount provided: {amount}")))
        } else {
            Ok(Money { amount })
        }
    }
}

pub struct Stock {
    ticker_symbol: String
}

impl Stock {
    pub fn new(ticker_symbol: String) -> Stock {
        Stock {ticker_symbol}
    }

    pub fn get_ticker_symbol(self) -> String {
        self.ticker_symbol
    }
}

pub struct Order {
    pub order_id: String,
    pub stock_id: Stock,
    pub amount: Money,
    pub order_type: OrderType,
    pub timestamp: SystemTime
}

pub enum OrderType {
    Buy,
    Sell
}


pub struct StockData {
    pub test: String
}

pub struct Config<'a> {
    pub alpha_vantage_api_key: &'a str
}