use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use serde_json::json;
use std::fmt;

#[derive(Debug)]
pub struct CustomError {
    pub error_status_code: StatusCode,
    pub error_message: String,
}

impl CustomError {
    pub fn new(error_status_code: StatusCode, error_message: String) -> Self {
        Self {
            error_status_code,
            error_message,
        }
    }
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.error_message.as_str())
    }
}

impl ResponseError for CustomError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.error_status_code).json(json!({ "message": self.error_message }))
    }
}