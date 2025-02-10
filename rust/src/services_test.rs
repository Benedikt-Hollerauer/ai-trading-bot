mod trading_api_service {
    use crate::errors::AppErrors;
    use crate::models::{Money, Order, OrderType, Stock, StockData, StockInvestment};
    use crate::services::TradingApiService;
    use crate::services::TradingApiServiceLive;
    use std::time::SystemTime;
    use tokio::test;

    #[test]
    async fn test_get_stock_data_method_success() {
        let maybe_stock_data: Result<StockData, AppErrors> =
            TradingApiServiceLive::get_stock_data(Stock::new("GOOG".to_string())).await;
        println!("{:?}", maybe_stock_data);
        assert!(maybe_stock_data.is_ok())
    }

    #[test]
    async fn test_get_stock_data_method_failure() {
        let maybe_stock_data: Result<StockData, AppErrors> =
            TradingApiServiceLive::get_stock_data(Stock::new("Not a stock".to_string())).await;
        assert!(maybe_stock_data.is_err())
    }

    #[test]
    async fn test_place_order_method_success() {
        let order_success_mock: Order = Order {
            stock_quantity: 1.1,
            stock: Stock::new("GOOG".to_string()),
            order_type: OrderType::Buy,
            timestamp: SystemTime::now(),
        };
        let maybe_successfully_placed_order: Result<String, AppErrors> =
            TradingApiServiceLive::place_order(order_success_mock);
        println!("{:?}", maybe_successfully_placed_order);
        assert!(maybe_successfully_placed_order.is_ok())
    }

    #[test]
    async fn test_place_order_method_failure() {
        let order_failure_mock: Order = Order {
            stock_quantity: 1.1,
            stock: Stock::new("not_a_stock".to_string()),
            order_type: OrderType::Buy,
            timestamp: SystemTime::now(),
        };
        let maybe_successfully_failed_order: Result<String, AppErrors> =
            TradingApiServiceLive::place_order(order_failure_mock);
        println!("{:?}", maybe_successfully_failed_order);
        assert!(maybe_successfully_failed_order.is_err())
    }

    #[test]
    async fn test_convert_money_amount_to_stock_quantity_method_success() {
        let money_mock = Money::new(1.1).unwrap();
        let maybe_stock_data: Result<f64, AppErrors> =
            TradingApiServiceLive::convert_money_amount_to_stock_quantity(
                money_mock,
                "GOOG".to_string(),
            );
        assert!(maybe_stock_data.is_ok())
    }

    #[test]
    async fn test_convert_money_amount_to_stock_quantity_method_failure() {
        let money_mock = Money::new(1.1).unwrap();
        let maybe_stock_data: Result<f64, AppErrors> =
            TradingApiServiceLive::convert_money_amount_to_stock_quantity(
                money_mock,
                "not_a_ticker_symbol".to_string(),
            );
        assert!(maybe_stock_data.is_err())
    }

    #[test]
    async fn test_get_quantity_to_sell_everything_method_success() {
        let maybe_stock_data: Result<f64, AppErrors> =
            TradingApiServiceLive::get_quantity_to_sell_everything("GOOG".to_string());
        println!("{:?}", maybe_stock_data);
        assert!(maybe_stock_data.is_ok())
    }

    #[test]
    async fn test_get_quantity_to_sell_everything_method_failure() {
        let maybe_stock_data: Result<f64, AppErrors> =
            TradingApiServiceLive::get_quantity_to_sell_everything(
                "not_a_ticker_symbol".to_string(),
            );
        assert!(maybe_stock_data.is_err())
    }

    #[test]
    async fn test_get_current_investment_success() {
        let maybe_current_investment: Result<StockInvestment, AppErrors> =
            TradingApiServiceLive::get_current_investment(Stock::new("GOOG".to_string()));
        println!("{:?}", maybe_current_investment);
        assert!(maybe_stock_data.is_ok())
    }

    #[test]
    async fn test_get_current_investment_failure() {
        let maybe_current_investment: Result<StockInvestment, AppErrors> =
            TradingApiServiceLive::get_current_investment(
                Stock::new("not_a_ticker_symbol".to_string()),
            );
        assert!(maybe_current_investment.is_err())
    }
}

mod ai_service {
    use crate::errors::AppErrors;
    use crate::models::{News, OrderType, Stock, StockData, StockPricePerformance};
    use crate::services::AiService;
    use crate::services::AiServiceLive;
    use tokio::test;

    #[test]
    async fn test_get_order_advice_method_success() {
        let test_stock_data: StockData = StockData {
            stock: Stock::new("GOOG".to_string()),
            stock_price_performance: vec![StockPricePerformance { date: "2017-12-29".to_string(), open: "1015.8".to_string(), high: "1078.49".to_string(), low: "988.28".to_string() }],
            news: vec![News {
                title: "Google's Fight Against Epic Games' Antitrust Win Hits Roadblock -Judges Tell Search Giant Apple Case Doesn't Apply - Alphabet  ( NASDAQ:GOOG ) , Apple  ( NASDAQ:AAPL ) ".to_string(),
                summary: "On Monday, a federal appeals court in San Francisco showed skepticism toward Alphabet Inc.'s GOOG GOOGL efforts to overturn a jury verdict in favor of Fortine-maker Epic Games. What Happened: The jury had sided with Epic in 2023, accusing Google of imposing restrictive policies on its Google Play ...".to_string(),
                time_published: "20250204T025520".to_string()
            }],
        };
        let maybe_order_advice: Result<OrderType, AppErrors> =
            AiServiceLive::get_order_advice(test_stock_data).await;
        assert!(maybe_order_advice.is_ok())
    }

    #[test]
    async fn test_get_order_advice_method_failure() {
        let test_stock_data: StockData = StockData {
            stock: Stock::new("GOOG".to_string()),
            stock_price_performance: vec![StockPricePerformance {
                date: "".to_string(),
                open: "".to_string(),
                high: "".to_string(),
                low: "".to_string(),
            }],
            news: vec![News {
                title: "".to_string(),
                summary: "".to_string(),
                time_published: "".to_string(),
            }],
        };
        let maybe_order_advice: Result<OrderType, AppErrors> =
            AiServiceLive::get_order_advice(test_stock_data).await;
        println!("{:?}", maybe_order_advice);
        assert!(maybe_order_advice.is_err())
    }
}