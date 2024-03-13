use crate::ApiServiceError;

pub trait ApiService {
    fn get_connection() -> Result<(), ApiServiceError>;
}