use errors::*;
use interfaces::*;

mod errors;
mod implementations;
mod interfaces;

fn main() {
    println!("Hello, world!");
}

fn compose(
    api_service: ApiService,
    api_service_error: ApiServiceError
) -> Result<(), TradingBotError> {
    Ok(())
}