use crate::errors::AppErrors;
use serde::Deserialize;
use std::string::ToString;
use std::time::SystemTime;

#[derive(Debug)]
pub struct Money {
    pub amount: f64,
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
            Err(AppErrors::ModelCreationError(format!(
                "The money amount can't be below 0. Amount provided: {amount}"
            )))
        } else if digits_count > 2 {
            Err(AppErrors::ModelCreationError(format!(
                "There were too many digits. Amount provided: {amount}"
            )))
        } else {
            Ok(Money { amount })
        }
    }
}

#[derive(Debug, Clone)]
pub struct Stock {
    pub ticker_symbol:  &'static str
}

#[derive(Debug)]
pub struct Order {
    pub stock_quantity: f64,
    pub stock: Stock,
    pub order_type: OrderType,
    pub timestamp: SystemTime,
}

#[derive(Debug)]
pub enum OrderType {
    Buy,
    Sell,
}

#[derive(Debug)]
pub struct StockPricePerformance {
    pub date: String,
    pub open: String,
    pub high: String,
    pub low: String,
}

#[derive(Deserialize, Debug)]
pub struct News {
    pub title: String,
    pub summary: String,
    pub time_published: String,
}

#[derive(Deserialize, Debug)]
pub struct NewsApiResponse {
    pub feed: Vec<News>,
}

#[derive(Debug)]
pub struct StockData {
    pub stock: Stock,
    pub stock_price_performance: Vec<StockPricePerformance>,
    pub news: Vec<News>,
}

pub struct Config<'a> {
    pub alpha_vantage_api_key: &'a str,
    pub interactive_brokers_connection_url_with_port: &'a str,
}

#[derive(Debug)]
pub struct StockInvestment {
    pub(crate) stock: Stock,
    pub(crate) stock_name: String,
    pub(crate) current_invested_amount: Money,
}