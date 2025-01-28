use crate::config::CONFIG;
use crate::errors::AppErrors;
use crate::models::{Money, News, NewsApiResponse, Order, OrderType, Stock, StockData, StockPricePerformance};
use alpha_vantage::stock_time::StockFunction;
use ibapi::accounts::AccountSummaryTags;
use ibapi::contracts::Contract;
use ibapi::market_data::historical::{ToDuration, BarSize, WhatToShow};
use ibapi::orders::Action;
use ibapi::Client as IbClient;
use reqwest::Client;

pub trait TradingApiService {
    async fn get_stock_data(stock_id: Stock) -> Result<StockData, AppErrors>;
    fn place_order(order: Order) -> Result<(), AppErrors>;
    fn convert_money_amount_to_stock_quantity(amount: Money, ticker_symbol: String) -> Result<i64, AppErrors>;
    fn get_quantity_to_sell_everything(ticker_symbol: String) -> Result<f64, AppErrors>;
}

pub struct TradingApiServiceLive;

pub trait AiService {
    fn get_order_advice(stock_data: StockData) -> Result<Order, AppErrors>;
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
                ticker_symbol: ticker_symbol.to_string(),
                stock_price_performance: stock_price_performance?,
                news: news?
            }
        )
    }

    fn place_order(order: Order) -> Result<(), AppErrors> {
        let connection_url = "";
        let contract = Contract::stock(&*order.stock.get_ticker_symbol());
        let action = match order.order_type {
            OrderType::Buy => Action::Buy,
            OrderType::Sell => Action::Sell
        };
        //let order = IbClient::connect(connection_url, 100)
        //    .map(|client: IbClient|
        //         (order_builder::market_order(
        //             action,
        //            1.0
        //         ), client)
        //    ).and_then(|(order, client)|
        //        client.place_order(
        //            client.next_order_id(),
        //            &contract,
        //            &order
        //        )
        //    ).map(|subscription|
        //        for event in subscription {
        //            if let PlaceOrder::ExecutionData(data) = event {
        //                println!("{:?}", data);
        //            } else {
        //                println!("{:?}", event);
        //            }
        //        }
        //    );
        todo!()
    }

    fn convert_money_amount_to_stock_quantity(amount: Money, ticker_symbol: String) -> Result<i64, AppErrors> {
        let connection_url = "127.0.0.1:4002";
        let contract = Contract::stock(&*ticker_symbol);
        let current_close = IbClient::connect(connection_url, 1)
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
                AppErrors::ConvertMoneyToStockQuantityError(error.to_string())
            ).and_then(|close|
                close.ok_or(AppErrors::ConvertMoneyToStockQuantityError("There was an error while trying to get the latest closing amount".to_string()))
            )?;
        Ok(
            (current_close / amount.amount).floor() as i64
        )
    }

    fn get_quantity_to_sell_everything(ticker_symbol: String) -> Result<f64, AppErrors> {
        let connection_url = "127.0.0.1:4002";
        let contract = Contract::stock(&*ticker_symbol);
        let client = IbClient::connect(connection_url, 1).expect("Connection to TWS failed!"); // TODO add error handeling
        let account_summary = client.account_summary("All", AccountSummaryTags::ALL).expect("Account summary failed!");
        print!("{:?}", account_summary);
        Ok(1.2)
    }
}

impl AiService for AiServiceLive {
    fn get_order_advice(stock_data: StockData) -> Result<Order, AppErrors> {
        todo!()
    }
}