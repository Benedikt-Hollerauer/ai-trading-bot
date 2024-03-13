pub trait TradingBotError {}

pub enum ApiServiceError {
    ConnectionFailed { error_message: str }
}