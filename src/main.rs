use crate::models::{Money, Order, OrderType};
use crate::services::{AiService, AiServiceLive, TradingApiService, TradingApiServiceLive};
use std::time::SystemTime;
use axum::handler::Handler;
use axum::http::HeaderMap;
use axum::response::{AppendHeaders, Html};
use axum::Router;
use axum::routing::get;

mod config;
mod errors;
mod models;
mod models_test;
mod services;
mod services_test;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handler))
        .route("/style.css", get(serve_css))
        .route("/app.js", get(serve_js));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
    //let order_process_result = async {
    //    let amount_to_invest = Money::new(1.1); //TODO implement through ui
    //    let stock = crate::services_test::INVESTED_PAPER_TRADING_STOCK; //TODO implement through ui
    //    let stock_data = TradingApiServiceLive::get_stock_data(stock.clone()).await?;
    //    let order_type = AiServiceLive::get_order_advice(stock_data).await?;
    //    let stock_quantity = match order_type {
    //        OrderType::Buy => TradingApiServiceLive::convert_money_amount_to_stock_quantity(
    //            amount_to_invest?,
    //            stock.clone(),
    //        ),
    //        OrderType::Sell => TradingApiServiceLive::get_quantity_to_sell_everything(
    //            stock.clone()
    //        )
    //    }?;
    //    let order = Order {
    //        stock_quantity: stock_quantity,
    //        stock: stock,
    //        order_type: order_type,
    //        timestamp: SystemTime::now(),
    //    };
    //    TradingApiServiceLive::place_order(order)
    //}.await;

    //match order_process_result {
    //    Ok(order) => println!("Process finished successfully. Order: {:?}", order),
    //    Err(app_error) => println!("Process failed with error: {:?}", app_error),
    //}
}

async fn handler() -> Html<&'static str> {
    Html(include_str!("../index.html"))
}

async fn serve_css() -> (HeaderMap, &'static str) {
    let mut headers = HeaderMap::new();
    headers.insert("content-type", "text/css".parse().unwrap());
    (headers, include_str!("../style.css"))
}

async fn serve_js() -> (HeaderMap, &'static str) {
    let mut headers = HeaderMap::new();
    headers.insert("content-type", "application/javascript".parse().unwrap());
    (headers, include_str!("../app.js"))
}