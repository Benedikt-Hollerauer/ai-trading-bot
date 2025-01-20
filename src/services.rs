use alpha_vantage::stock_time::StockFunction;
use crate::errors::AppErrors;
use crate::models::{Order, StockData, Stock};
use crate::config::config;

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
        let api_key = alpha_vantage::set_api(config.alpha_vantage_api_key, reqwest::Client::new());
        let test = api_key.stock_time(StockFunction::Monthly, &stock.get_ticker_symbol());
        println!("{:?}", test.json().await.unwrap());
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