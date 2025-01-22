use std::error::Error;

#[derive(Debug)]
pub enum AppErrors {
    ModelCreationError(String),
    GetStockDataError(String),
}