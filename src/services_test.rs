mod TradingApiService {
    use std::time::SystemTime;
    use crate::errors::AppErrors;
    use crate::services::TradingApiServiceLive;
    use crate::services::TradingApiService;
    use crate::models::{Money, Order, OrderType, StockData, Stock};

    #[test]
    fn test_get_stock_data_method_success() {
        let maybe_stock_data: Result<StockData, AppErrors> = TradingApiServiceLive::get_stock_data(Stock::new("GOOG".to_string()));
        assert!(maybe_stock_data.is_ok())
    }

    #[test]
    fn test_get_stock_data_method_failure() {
        let maybe_stock_data: Result<StockData, AppErrors> = TradingApiServiceLive::get_stock_data(Stock::new("Not a stock".to_string()));
        assert!(maybe_stock_data.is_err())
    }

    #[test]
    fn test_place_order_method_success() {
        let order_success_mock: Order = Order {
            order_id: "1".to_string(),
            stock_id: Stock::new("GOOG".to_string()),
            amount: Money::new(1.1).unwrap(),
            order_type: OrderType::Buy,
            timestamp: SystemTime::now()
        };
        let maybe_stock_data: Result<bool, AppErrors> = TradingApiServiceLive::place_order(order_success_mock);
        assert!(maybe_stock_data.is_ok())
    }

    #[test]
    fn test_place_order_method_failure() {
        let order_failure_mock: Order = Order {
            order_id:  "1".to_string(),
            stock_id: Stock::new("GOOG".to_string()),
            amount: Money::new(1.1).unwrap(),
            order_type: OrderType::Buy,
            timestamp: SystemTime::now()
        };
        let maybe_stock_data: Result<bool, AppErrors> = TradingApiServiceLive::place_order(order_failure_mock);
        assert!(maybe_stock_data.is_err())
    }
}

mod AiService {
    use crate::errors::AppErrors;
    use crate::models::{Order, StockData};
    use crate::services::AiServiceLive;


    #[test]
    fn test_get_order_advice_method_success() {
        let test_stock_data: StockData = StockData {
            test: "test".to_string()
        };
        let maybe_order_advice: Result<Order, AppErrors> = AiServiceLive::get_order_advice(test_stock_data);
        assert!(maybe_order_advice.is_ok())
    }

    #[test]
    fn test_get_order_advice_method_failure() {
        let test_stock_data: StockData = StockData {
            test: "test".to_string()
        };
        let maybe_order_advice: Result<Order, AppErrors> = AiServiceLive::get_order_advice(test_stock_data);
        assert!(maybe_order_advice.is_err())
    }
}