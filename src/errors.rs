
#[derive(Debug)]
pub enum AppErrors {
    ModelCreationError(String),
    GetStockDataError(String),
    ConvertMoneyToStockQuantityError(String),
}