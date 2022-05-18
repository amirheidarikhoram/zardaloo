use super::dtos::{request::LoginRequestDTO, response::LoginResponseDTO};
use crate::lib::auth::token;
use crate::lib::CustomError;

use actix_web::{
    http::StatusCode,
    post,
    web::{scope, Json, ServiceConfig},
    HttpResponse,
};

#[post("/login")]
async fn login(data: Json<LoginRequestDTO>) -> Result<HttpResponse, CustomError> {
    let data = data.into_inner();
    let token = token::sign(data.username, data.password)?;

    return Ok(HttpResponse::build(StatusCode::OK).json(LoginResponseDTO { token }));
}

pub fn init_routes(config: &mut ServiceConfig) {
    config.service(scope("/users").service(login));
}
