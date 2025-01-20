use crate::errors::AppErrors;
use crate::models::{Order, StockData, Stock};
use crate::config::config;
use alphavantage::blocking::Client;

pub trait TradingApiService {
    fn get_stock_data(stock_id: Stock) -> Result<StockData, AppErrors>;
    fn place_order(order: Order) -> Result<bool, AppErrors>;
}

pub struct TradingApiServiceLive;

pub trait AiService {
    fn get_order_advice(stock_data: StockData) -> Result<Order, AppErrors>;
}

pub struct AiServiceLive;

impl TradingApiService for TradingApiServiceLive {
    fn get_stock_data(stock: Stock) -> Result<StockData, AppErrors> {
        let client = Client::new(config.alpha_vantage_api_key);
        let time_series = client
            .get_time_series_monthly(&*stock.get_ticker_symbol())
            .unwrap(); // TODO error handeling
        println!("{:?}", time_series);
        Ok(
            StockData {test: "".to_string()}
        )
    }

    fn place_order(order: Order) -> Result<bool, AppErrors> {
        todo!()
    }
}

impl AiService for AiServiceLive {
    fn get_order_advice(stock_data: StockData) -> Result<Order, AppErrors> {
        todo!()
    }
}