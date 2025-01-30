use crate::errors::AppErrors;
use serde::Deserialize;
use std::string::ToString;
use std::time::SystemTime;

pub struct Money {
    pub amount: f64
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
    pub stock: Stock,
    pub order_type: OrderType,
    pub timestamp: SystemTime
}

pub enum OrderType {
    Buy(Money),
    Sell
}

#[derive(Debug)]
pub struct StockPricePerformance {
    pub date: String,
    pub open: String,
    pub high: String,
    pub low: String
}

#[derive(Deserialize, Debug)]
pub struct News {
    pub title: String,
    pub summary: String,
    pub time_published: String
}

#[derive(Deserialize, Debug)]
pub struct NewsApiResponse {
    pub feed: Vec<News>,
}

#[derive(Debug)]
pub struct StockData {
    pub ticker_symbol: String,
    pub stock_price_performance: Vec<StockPricePerformance>,
    pub news: Vec<News>
}

pub struct Config<'a> {
    pub alpha_vantage_api_key: &'a str,
    pub interactive_brokers_connection_url_with_port: &'a str
}