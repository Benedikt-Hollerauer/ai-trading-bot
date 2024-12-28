use std::time::SystemTime;

struct Money {
    amount: f64
}

struct Order {
    order_id: String,
    amount: Money,
    order_type: OrderType,
    timestamp: SystemTime
}

enum OrderType {
    Buy,
    Sell
}