use std::time::SystemTime;
use flutter_rust_bridge::frb;
use crate::models::{Money, News, NewsApiResponse, Order, OrderType, Stock, StockData, StockInvestment, StockPricePerformance};
use crate::services::{AiService, AiServiceLive, TradingApiService, TradingApiServiceLive};

#[flutter_rust_bridge::frb(sync)] // Synchronous mode for simplicity of the demo
pub fn greet(name: String) -> String {
    format!("Hello, {name}!")
}

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    // Default utilities - feel free to customize
    flutter_rust_bridge::setup_default_user_utils();
}

// Mirrors (https://cjycode.com/flutter_rust_bridge/guides/third-party/manual)

#[frb(mirror_all)]
mod external_types {
    pub use alpha_vantage::stock_time::StockFunction;
    pub use ibapi::accounts::PositionUpdate;
    pub use ibapi::contracts::Contract;
    pub use ibapi::market_data::historical::{BarSize, ToDuration, WhatToShow};
    pub use ibapi::orders::{Action, order_builder};
    pub use ibapi::Client as IbClient;
    pub use ollama_rs::generation::completion::GenerationResponse;
    pub use ollama_rs::generation::options::GenerationOptions;
    pub use ollama_rs::Ollama;
    pub use reqwest::Client;
}

#[frb(mirror_all)]
impl Money {}

#[frb(mirror_all)]
impl News {}

#[frb(mirror_all)]
impl NewsApiResponse {}

#[frb(mirror_all)]
impl Order {}

#[frb(mirror_all)]
impl OrderType {}

#[frb(mirror_all)]
impl Stock {}

#[frb(mirror_all)]
impl StockData {}

#[frb(mirror_all)]
impl StockInvestment {}

#[frb(mirror_all)]
impl StockPricePerformance {}

// Mirrors end

#[cfg(target_arch = "wasm32")]
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
#[cfg(target_arch = "wasm32")]
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