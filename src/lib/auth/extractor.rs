use crate::lib::CustomError;

use actix_web::{
    dev::Payload,
    http::{header, StatusCode},
    // web::PayloadConfig,
    FromRequest, HttpRequest,
};

use serde::Serialize;
use std::future::{ready, Ready};

#[derive(Serialize)]
pub struct AuthData {
    pub authorized: bool,
}

impl AuthData {
    pub fn new(authorized: bool) -> Self {
        Self { authorized }
    }
}

impl FromRequest for AuthData {
    type Error = CustomError;
    type Future = Ready<Result<AuthData, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        // check if token of header is valid
        if req.headers().contains_key(header::AUTHORIZATION) {
            let auth_token: Vec<String> = req
                .headers()
                .get(header::AUTHORIZATION)
                .unwrap()
                .to_str()
                .unwrap()
                .split(" ")
                .map(|x| x.to_string())
                .collect::<Vec<String>>();
            if auth_token.len() == 2 && auth_token[0] == "Bearer".to_string() {
                let token = &auth_token[1];

                match crate::lib::auth::token::verify(token.clone()) {
                    Ok(status) => {
                        let auth_data = AuthData::new(
                            status,
                        );
                        return ready(Ok(auth_data));
                    }
                    Err(err) => {
                        return ready(Err(err));
                    }
                }
            } else {
                ready(Err(CustomError::new(
                    StatusCode::UNAUTHORIZED,
                    "Bad authorization header structure".to_string(),
                )))
            }
        } else {
            ready(Err(CustomError::new(
                StatusCode::UNAUTHORIZED,
                "No authorization header provided".to_string(),
            )))
        }
    }
}
