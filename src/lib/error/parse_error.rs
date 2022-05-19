use std::num::{ParseFloatError, ParseIntError};

use actix_web::http::StatusCode;

use super::{CustomError, ToCustomErrorTrait};

impl<T> ToCustomErrorTrait<T, ParseFloatError> for Result<T, ParseFloatError> where T: Clone{
    fn to_custom_error(&self, message: &str) -> Result<T, CustomError> {
        if let Ok(parsed_value) = self {
            Ok(parsed_value.clone())
        } else {
            Err(CustomError::new(
                StatusCode::BAD_REQUEST,
                message.to_string(),
            ))
        }
    }
}

impl<T: Copy + Clone> ToCustomErrorTrait<T, ParseIntError> for Result<T, ParseIntError>{
    fn to_custom_error(&self, message: &str) -> Result<T, CustomError> {
        if let Ok(parsed_value) = self {
            Ok(parsed_value.clone())
        } else {
            Err(CustomError::new(
                StatusCode::BAD_REQUEST,
                message.to_string(),
            ))
        }
    }
}
