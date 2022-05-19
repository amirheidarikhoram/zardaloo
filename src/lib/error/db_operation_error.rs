use actix_web::http::StatusCode;

use super::{CustomError, ToCustomErrorTrait};

impl<T> ToCustomErrorTrait<T, String> for Result<T, String> where T:Clone {
    fn to_custom_error(&self, message: &str) -> Result<T, CustomError> {
        if let Ok(value) = self {
            Ok(value.clone())
        } else {
            Err(CustomError::new(
                StatusCode::BAD_REQUEST,
                message.to_string(),
            ))
        }
    }
}
