use std::future;
use std::time::SystemTime;
use crate::config::CONFIG;
use crate::errors::AppErrors;
use crate::models::{Money, News, NewsApiResponse, Order, OrderType, Stock, StockData, StockPricePerformance};
use alpha_vantage::stock_time::StockFunction;
use ibapi::accounts::{AccountSummaryTags, Position, PositionUpdate};
use ibapi::contracts::Contract;
use ibapi::market_data::historical::{ToDuration, BarSize, WhatToShow};
use ibapi::orders::{order_builder, Action, PlaceOrder};
use ibapi::Client as IbClient;
use ollama_rs::generation::completion::request::GenerationRequest;
use ollama_rs::generation::options::GenerationOptions;
use ollama_rs::Ollama;
use reqwest::Client;

pub trait TradingApiService {
    async fn get_stock_data(stock_id: Stock) -> Result<StockData, AppErrors>;
    fn place_order(order: Order) -> Result<(), AppErrors>;
    fn convert_money_amount_to_stock_quantity(amount: Money, ticker_symbol: String) -> Result<f64, AppErrors>;
    fn get_quantity_to_sell_everything(ticker_symbol: String) -> Result<f64, AppErrors>;
}

pub struct TradingApiServiceLive;

pub trait AiService {
    async fn get_order_advice(money_amount_to_buy: Money, stock_data: StockData) -> Result<Order, AppErrors>;
}

pub struct AiServiceLive;

impl TradingApiService for TradingApiServiceLive {
    async fn get_stock_data(stock: Stock) -> Result<StockData, AppErrors> {
        let ticker_symbol= stock.get_ticker_symbol();
        let api_key = alpha_vantage::set_api(CONFIG.alpha_vantage_api_key, reqwest::Client::new());
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

        let client = Client::new();
        let url = "https://www.alphavantage.co/query";
        let params = [
            ("function", "NEWS_SENTIMENT"),
            ("tickers", &*ticker_symbol),
            ("apikey", CONFIG.alpha_vantage_api_key),
        ];

        let response = client.get(url)
            .query(&params)
            .send()
            .await;

        let news: Result<Vec<News>, AppErrors> = match response {
            Ok(news_response) =>
                match news_response.json::<NewsApiResponse>().await {
                    Ok(news_api_response) => Ok(news_api_response.feed),
                    Err(error) => Err(AppErrors::GetStockDataError(error.to_string()))
                }
            Err(error) => Err(AppErrors::GetStockDataError(error.to_string())),
        };

        Ok(
            StockData {
                stock: Stock::new(ticker_symbol.to_string()),
                stock_price_performance: stock_price_performance?,
                news: news?
            }
        )
    }

    fn place_order(order: Order) -> Result<(), AppErrors> {
        let ticker = order.stock.get_ticker_symbol();
        let contract = Contract::stock(&*ticker);

        let mut client = IbClient::connect(
            CONFIG.interactive_brokers_connection_url_with_port,
            1
        ).map_err(|e| AppErrors::PlaceOrderError(e.to_string()))?;

        let order_id = client.next_order_id(); // Now using mutable borrow

        let action = match order.order_type {
            OrderType::Buy(amount) => (Action::Buy, Some(amount)),
            OrderType::Sell => (Action::Sell, None)
        };

        let quantity = match action {
            (Action::Buy, Some(amount)) => Self::convert_money_amount_to_stock_quantity(amount, ticker),
            (Action::Sell, _) => Self::get_quantity_to_sell_everything(ticker),
            _ => Err(AppErrors::PlaceOrderError("Error while getting the quantity.".to_string()))
        }?;

        let order = order_builder::market_order(action.0, quantity);

        client.place_order(order_id, &contract, &order)
            .map(|_| ())
            .map_err(|e| AppErrors::PlaceOrderError(e.to_string()))
    }

    fn convert_money_amount_to_stock_quantity(amount: Money, ticker_symbol: String) -> Result<f64, AppErrors> {
        let contract = Contract::stock(&*ticker_symbol);
        let current_close = IbClient::connect(CONFIG.interactive_brokers_connection_url_with_port, 1)
            .and_then(|client: IbClient| client
                .historical_data(&contract, None, 1.days(), BarSize::Day, WhatToShow::Trades, true)
                .and_then(|historical_data|
                    Ok(
                        historical_data
                            .bars
                            .iter()
                            .next()
                            .map(|bar| bar.close)
                    )
                )
            ).map_err(|error|
                AppErrors::ConvertMoneyToStockQuantityError(error.to_string() + " for ticker: "+ &ticker_symbol)
            ).and_then(|close|
                close.ok_or(AppErrors::ConvertMoneyToStockQuantityError("There was an error while trying to get the latest closing amount".to_string()))
            )?;
        Ok(
            (current_close / amount.amount).floor()
        )
    }

    fn get_quantity_to_sell_everything(ticker_symbol: String) -> Result<f64, AppErrors> {
        let client = IbClient::connect(CONFIG.interactive_brokers_connection_url_with_port, 1)
            .map_err(|error| AppErrors::GetQuantityToSellEverythingError(error.to_string()))?;
        let positions = client.positions()
            .map_err(|error| AppErrors::GetQuantityToSellEverythingError(error.to_string()))?;

        let position = positions.iter()
            .take_while(|position_update|
                !matches!(position_update, PositionUpdate::PositionEnd)
            ).find(|position_update|
                match position_update {
                    PositionUpdate::Position(position) => position.contract.symbol == ticker_symbol,
                    _ => false
                }
            ).and_then(|position_update|
                match position_update {
                    PositionUpdate::Position(position) => Some(position),
                    _ => None
                }
            ).ok_or(
                AppErrors::GetQuantityToSellEverythingError(
                    "There was an error while trying to get the latest closing amount. Possibly there are no positions available or not the position with this ticker_symbol: ".to_string() + &*ticker_symbol
                )
            )?;
        Ok(
            position.position.abs()
        )
    }
}

impl AiService for AiServiceLive {
    async fn get_order_advice(
        money_amount_to_buy: Money,
        stock_data: StockData
    ) -> Result<Order, AppErrors> {
        let ollama = Ollama::default();
        let model = "deepseek-r1:1.5b".to_string();
        let options = GenerationOptions::default()
            .temperature(0.0);
        let ticker_symbol = stock_data.stock
            .clone()
            .get_ticker_symbol();
        let prompt = format!(
            "Portfolio analysis:\nTicker: {}\nNews: {:?}\nPrice: {:?}\nShould I SELL or BUY? Reply with only one word: SELL or BUY. (If you are not sure, do not reply with one of those words)",
            ticker_symbol,
            stock_data.news,
            stock_data.stock_price_performance
        );

        let order_advice_result = ollama
            .generate(GenerationRequest::new(model, prompt).options(options))
            .await;

        let order_type = order_advice_result
            .map_err(|error| AppErrors::GetOrderAdviceError(error.to_string()))
            .and_then(|ai_result|
                if ai_result.response.contains("SELL") {
                    Ok(OrderType::Sell)
                } else if ai_result.response.contains("BUY") {
                    Ok(OrderType::Buy(money_amount_to_buy))
                } else {
                    Err(AppErrors::GetOrderAdviceError("The Ai didn't respond with a clear order advice".to_string()))
                }
            )?;
        Ok(
            Order {
                stock: stock_data.stock,
                order_type: order_type,
                timestamp: SystemTime::now()
            }
        )
    }
}