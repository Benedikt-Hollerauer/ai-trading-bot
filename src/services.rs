use std::time::SystemTime;
use alpha_vantage::stock_time::StockFunction;
use crate::errors::AppErrors;
use crate::models::{Order, StockData, Stock, StockPricePerformance};
use crate::config::config;

pub trait TradingApiService {
    async fn get_stock_data(stock_id: Stock) -> Result<StockData, AppErrors>;
    fn place_order(order: Order) -> Result<bool, AppErrors>;
}

pub struct TradingApiServiceLive;

pub trait AiService {
    fn get_order_advice(stock_data: StockData) -> Result<Order, AppErrors>;
}

pub struct AiServiceLive;

impl TradingApiService for TradingApiServiceLive {
    async fn get_stock_data(stock: Stock) -> Result<StockData, AppErrors> {
        let ticker_symbol= stock.get_ticker_symbol();
        let api_key = alpha_vantage::set_api(config.alpha_vantage_api_key, reqwest::Client::new());
        let time_series = api_key.stock_time(StockFunction::Monthly, &ticker_symbol)
            .json()
            .await;

        let stock_price_performance: Result<Vec<StockPricePerformance>, AppErrors> = match time_series {
            Ok(time_series) => Ok(
                time_series.data()
                    .iter()
                    .map(|stock_price| StockPricePerformance {
                        date: stock_price.time().to_string(),
                        high: stock_price.high().to_string(),
                        low: stock_price.low().to_string(),
                        open: stock_price.open().to_string()
                    }).collect()
            ),
            Err(error) => Err(AppErrors::GetStockDataError(error.to_string())),
        };

        let earnings = api_key.custom("NEWS_SENTIMENT")
            .json::<String>()
            .await;

        let earning_reports = match earnings {
            Ok(earning_reports) => Ok(
                earning_reports
            ),
            Err(error) => Err(AppErrors::GetStockDataError(error.to_string()))
        };

        println!("{:?}", earning_reports);

        Ok(
            StockData {
                ticker_symbol: ticker_symbol.to_string(),
                stock_price_performance: stock_price_performance?,
                news: todo!()
            }
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