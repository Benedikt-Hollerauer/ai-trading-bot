mod trading_api_service {
    use crate::errors::AppErrors;
    use crate::models::{Money, Order, OrderType, Stock, StockData};
    use crate::services::TradingApiService;
    use crate::services::TradingApiServiceLive;
    use std::time::SystemTime;
    use tokio::test;

    #[test]
    async fn test_get_stock_data_method_success() {
        let maybe_stock_data: Result<StockData, AppErrors> = TradingApiServiceLive::get_stock_data(Stock::new("GOOG".to_string())).await;
        println!("{:?}", maybe_stock_data);
        //assert!(maybe_stock_data.is_ok())
    }

    #[test]
    async fn test_get_stock_data_method_failure() {
        let maybe_stock_data: Result<StockData, AppErrors> = TradingApiServiceLive::get_stock_data(Stock::new("Not a stock".to_string())).await;
        assert!(maybe_stock_data.is_err())
    }

    #[test]
    async fn test_place_order_method_success() {
        let order_success_mock: Order = Order {
            order_id: "1".to_string(),
            stock: Stock::new("GOOG".to_string()),
            amount: Money::new(1.1).unwrap(),
            order_type: OrderType::Buy,
            timestamp: SystemTime::now()
        };
        let maybe_stock_data: Result<(), AppErrors> = TradingApiServiceLive::place_order(order_success_mock);
        assert!(maybe_stock_data.is_ok())
    }

    #[test]
    async fn test_place_order_method_failure() {
        let order_failure_mock: Order = Order {
            order_id:  "1".to_string(),
            stock: Stock::new("GOOG".to_string()),
            amount: Money::new(1.1).unwrap(),
            order_type: OrderType::Buy,
            timestamp: SystemTime::now()
        };
        let maybe_stock_data: Result<(), AppErrors> = TradingApiServiceLive::place_order(order_failure_mock);
        assert!(maybe_stock_data.is_err())
    }

    #[test]
    async fn test_convert_money_amount_to_stock_quantity_method_success() {
        let money_mock = Money::new(1.1).unwrap();
        let maybe_stock_data: Result<i64, AppErrors> = TradingApiServiceLive::convert_money_amount_to_stock_quantity(money_mock, "GOOG".to_string());
        println!("{:?}", maybe_stock_data);
        //assert!(maybe_stock_data.is_ok())
    }

    #[test]
    async fn test_convert_money_amount_to_stock_quantity_method_failure() {
        let money_mock = Money::new(1.1).unwrap();
        let maybe_stock_data: Result<i64, AppErrors> = TradingApiServiceLive::convert_money_amount_to_stock_quantity(money_mock, "not_a_ticker_symbol".to_string());
        assert!(maybe_stock_data.is_err())
    }

    #[test]
    async fn test_get_quantity_to_sell_everything_method_success() {
        let maybe_stock_data: Result<f64, AppErrors> = TradingApiServiceLive::get_quantity_to_sell_everything("GOOG".to_string());
        println!("{:?}", maybe_stock_data);
        //assert!(maybe_stock_data.is_ok())
    }

    #[test]
    async fn test_get_quantity_to_sell_everything_method_failure() {
        let maybe_stock_data: Result<f64, AppErrors> = TradingApiServiceLive::get_quantity_to_sell_everything("not_a_ticker_symbol".to_string());
        assert!(maybe_stock_data.is_err())
    }
}

mod ai_service {
    use crate::errors::AppErrors;
    use crate::models::{Order, StockData};
    use crate::services::AiService;
    use crate::services::AiServiceLive;


    #[test]
    fn test_get_order_advice_method_success() {
        let test_stock_data: StockData = StockData {
            ticker_symbol: "GOOG".to_string(),
            stock_price_performance: todo!(),
            news: todo!()
        };
        let maybe_order_advice: Result<Order, AppErrors> = AiServiceLive::get_order_advice(test_stock_data);
        assert!(maybe_order_advice.is_ok())
    }

    #[test]
    fn test_get_order_advice_method_failure() {
        let test_stock_data: StockData = StockData {
            ticker_symbol: "GOOG".to_string(),
            stock_price_performance: todo!(),
            news: todo!()
        };
        let maybe_order_advice: Result<Order, AppErrors> = AiServiceLive::get_order_advice(test_stock_data);
        assert!(maybe_order_advice.is_err())
    }
}