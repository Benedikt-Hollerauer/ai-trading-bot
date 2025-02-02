
#[derive(Debug)]
pub enum AppErrors {
    ModelCreationError(String),
    GetStockDataError(String),
    ConvertMoneyToStockQuantityError(String),
    GetQuantityToSellEverythingError(String),
    PlaceOrderError(String),
    GetOrderAdviceError(String)
}