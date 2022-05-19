use std::sync::mpsc::SendError;

use actix_web::http::StatusCode;

use crate::lib::LifeIndicator;

use super::{CustomError, ToCustomErrorTrait};

impl ToCustomErrorTrait<(), SendError<LifeIndicator>> for Result<(), SendError<LifeIndicator>>{
    fn to_custom_error(&self, message: &str) -> Result<(), CustomError> {
        if let Ok(_) = self {
            Ok(())
        } else {
            Err(CustomError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                message.to_string(),
            ))
        }
    }
}