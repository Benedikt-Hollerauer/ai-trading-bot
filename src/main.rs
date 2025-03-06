use crate::models::{Money, Order, OrderType, Stock};
use crate::services::{AiService, AiServiceLive, TradingApiService, TradingApiServiceLive};
use axum::handler::Handler;
use axum::http::HeaderMap;
use axum::response::Html;
use axum::routing::{get, post};
use axum::{Json, Router};
use std::time::SystemTime;

mod config;
mod errors;
mod models;
mod models_test;
mod services;
mod services_test;

use axum::extract::State;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

// Add these structs
#[derive(Debug, Clone, Deserialize, Serialize)]
struct AnalysisRequest {
    ticker: String,
    amount: f64,
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    error_type: String,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<String>,
}

#[derive(Debug, Serialize)]
struct AnalysisResponse {
    message: String,
    order_type: String,
    quantity: f64,
    price: f64,
}

#[derive(Clone)]
struct AppState {
    trading_service: Arc<dyn TradingApiService + Send + Sync>,
    ai_service: Arc<dyn AiService + Send + Sync>,
}

#[tokio::main]
async fn main() {
    let state = AppState {
        trading_service: Arc::new(TradingApiServiceLive),
        ai_service: Arc::new(AiServiceLive),
    };

    let app = Router::new()
        .route("/", get(handler))
        .route("/analyze", post(analyze_investment))
        .route("/style.css", get(serve_css))
        .route("/app.js", get(serve_js))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Listening on http://{}", listener.local_addr().unwrap());

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

// Add this handler function
async fn analyze_investment(
    State(state): State<AppState>,
    Json(payload): Json<AnalysisRequest>,
) -> Result<Json<AnalysisResponse>, Json<ErrorResponse>> {
    let analysis_request = payload.clone();
    let ticker_symbol = &*analysis_request.ticker.clone();

    let stock = Stock {
        ticker_symbol: analysis_request.ticker.clone()
    };

    let stock_data = state.trading_service.get_stock_data(stock.clone())
        .await
        .map_err(|e| Json(ErrorResponse {
            error_type: "DATA_FETCH_FAILED".into(),
            message: "Failed to retrieve stock data".into(),
            details: Some(format!("{:?}", e)),
        }))?;

    println!("_________________________");

    let order_type = state.ai_service.get_order_advice(stock_data)
        .await
        .map_err(|e| Json(ErrorResponse {
            error_type: "ORDER_ADVICE_FETCH_FAILED".into(),
            message: "Failed to retrieve order advice".into(),
            details: Some(format!("{:?}", e)),
        }))?;

    let quantity = match order_type {
        OrderType::Buy => state.trading_service.convert_money_amount_to_stock_quantity(
            Money::new(payload.amount).map_err(|e| Json(ErrorResponse {
                error_type: "CONVERTING_MONEY_TO_STOCK_QUANTITY_FAILED".into(),
                message: "Failed to convert to stock quantity".into(),
                details: Some(format!("{:?}", e)),
            }))?,
            stock.clone(),
        ),
        OrderType::Sell => state.trading_service.get_quantity_to_sell_everything(
            stock.clone()
        )
    }.map_err(|e| Json(ErrorResponse {
        error_type: "GETTING_THE_QUANTITY_FAILED".into(),
        message: "Failed to get the quantity".into(),
        details: Some(format!("{:?}", e)),
    }))?;

    let order = Order {
        stock_quantity: quantity,
        stock: Stock { ticker_symbol: ticker_symbol.to_string() },
        order_type,
        timestamp: SystemTime::now(),
    };

    state.trading_service.place_order(order.clone())
        .map_err(|e| Json(ErrorResponse {
            error_type: "PLACING_THE_ORDER_FAILED".into(),
            message: "Failed to place the order".into(),
            details: Some(format!("Error: {:?}, Order: {:?}", e, order)),
        }))?;

    Ok(Json(AnalysisResponse {
        message: "Analysis complete".to_string(),
        order_type: format!("{:?}", order.order_type),
        quantity: order.stock_quantity,
        price: 1.1, //stock_data.stock_price_performance.current_price, //TODO this has to be different
    }))
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