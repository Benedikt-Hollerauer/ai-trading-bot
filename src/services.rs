use crate::errors::AppErrors;
use crate::models::{Order, StockData, StockId};

pub trait TradingApiService {
    fn get_stock_data(stock_id: StockId) -> Result<StockData, AppErrors>;
    fn place_order(order: Order) -> Result<bool, AppErrors>;
}

pub struct TradingApiServiceLive;

pub trait AiService {
    fn get_order_advice(stock_data: StockData) -> Result<Order, AppErrors>;
}

pub struct AiServiceLive;

impl TradingApiService for TradingApiServiceLive {
    fn get_stock_data(stock_id: StockId) -> Result<StockData, AppErrors> {

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