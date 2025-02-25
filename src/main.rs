use crate::models::{Money, Order, OrderType};
use crate::services::{AiService, AiServiceLive, TradingApiService, TradingApiServiceLive};
use std::time::SystemTime;
use axum::response::Html;
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
    // build our application with a route
    let app = Router::new().route("/", get(handler));

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
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
    Html("<h1>Hello, World!</h1>")
}