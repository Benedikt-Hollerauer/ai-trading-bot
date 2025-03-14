use crate::models::{Money, News, NewsApiResponse, Order, OrderType, Stock, StockData, StockInvestment, StockPricePerformance};
use crate::services::{AiService, AiServiceLive, TradingApiService, TradingApiServiceLive};
use flutter_rust_bridge::frb;
use std::time::SystemTime;

pub fn get_current_investment(
    ticker_symbol: &'static str,
) -> Result<StockInvestment, String> {
    let stock = Stock {
        ticker_symbol: ticker_symbol
    };
    TradingApiServiceLive::get_current_investment(stock)
        .map_err(|app_error|
            format!("There was an error while trying to fetch the current investment: {:?}", app_error)
        )
}

//#[tokio::main]
pub async fn execute_process(
    ticker_symbol: &'static str,
    money_amount_to_invest: f64,
) -> String {
    let order_process_result = async {
        let amount_to_invest = Money::new(money_amount_to_invest);
        let stock = Stock {
            ticker_symbol: &ticker_symbol,
        };
        let stock_data = TradingApiServiceLive::get_stock_data(stock.clone()).await?;
        let order_type = AiServiceLive::get_order_advice(stock_data).await?;
        let stock_quantity = match order_type {
            OrderType::Buy => TradingApiServiceLive::convert_money_amount_to_stock_quantity(
                amount_to_invest?,
                stock.clone(),
            ),
            OrderType::Sell => TradingApiServiceLive::get_quantity_to_sell_everything(
                stock.clone()
            )
        }?;
        let order = Order {
            stock_quantity: stock_quantity,
            stock: stock,
            order_type: order_type,
            timestamp: SystemTime::now(),
        };
        TradingApiServiceLive::place_order(order)
    }.await;
    match order_process_result {
        Ok(order) => format!("Process finished successfully. Order: {:?}", order),
        Err(app_error) => format!("Process failed with error: {:?}", app_error),
    }
}