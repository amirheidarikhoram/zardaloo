use super::dtos::{DataResponseDTO, DeleteRequestDTO, GetRequestDTO, SetRequestDTO};
use crate::lib::auth::AuthData;
use crate::lib::{CustomError, ToCustomErrorTrait};
use crate::AppState;
use zardaloo_db::*;

use actix_web::{
    delete, get,
    http::StatusCode,
    post,
    web::{scope, Data, Json, ServiceConfig},
    HttpResponse,
};

#[get("")]
async fn get_value(
    body: Json<GetRequestDTO>,
    _auth: AuthData,
    state: Data<AppState>,
) -> Result<HttpResponse, CustomError> {
    let body = body.into_inner();
    let data = state.db.lock().unwrap();
    let requested_type = body.id.split("#").collect::<Vec<&str>>();
    if requested_type.len() != 2 {
        return Err(CustomError::new(
            StatusCode::BAD_REQUEST,
            "Id is not valid".to_string(),
        ));
    }
    let requested_type = requested_type[1];
    match requested_type {
        "str" => {
            let value: Result<DbValue<String>, String> = data.get(body.id);
            if let Ok(value) = value {
                return Ok(HttpResponse::build(StatusCode::OK).json(DataResponseDTO {
                    value: value.value,
                    lifetime: value.lifetime,
                    id: value.id,
                }));
            }
        }
        "i32" => {
            let value: Result<DbValue<i32>, String> = data.get(body.id);
            if let Ok(value) = value {
                return Ok(HttpResponse::build(StatusCode::OK).json(DataResponseDTO {
                    value: value.value,
                    lifetime: value.lifetime,
                    id: value.id,
                }));
            }
        }
        "i64" => {
            let value: Result<DbValue<i64>, String> = data.get(body.id);
            if let Ok(value) = value {
                return Ok(HttpResponse::build(StatusCode::OK).json(DataResponseDTO {
                    value: value.value,
                    lifetime: value.lifetime,
                    id: value.id,
                }));
            }
        }
        "i128" => {
            let value: Result<DbValue<i128>, String> = data.get(body.id);
            if let Ok(value) = value {
                return Ok(HttpResponse::build(StatusCode::OK).json(DataResponseDTO {
                    value: value.value,
                    lifetime: value.lifetime,
                    id: value.id,
                }));
            }
        }
        "f32" => {
            let value: Result<DbValue<f32>, String> = data.get(body.id);
            if let Ok(value) = value {
                return Ok(HttpResponse::build(StatusCode::OK).json(DataResponseDTO {
                    value: value.value,
                    lifetime: value.lifetime,
                    id: value.id,
                }));
            }
        }
        "f64" => {
            let value: Result<DbValue<f64>, String> = data.get(body.id);
            if let Ok(value) = value {
                return Ok(HttpResponse::build(StatusCode::OK).json(DataResponseDTO {
                    value: value.value,
                    lifetime: value.lifetime,
                    id: value.id,
                }));
            }
        }
        _ => {
            return Err(CustomError::new(
                StatusCode::BAD_REQUEST,
                "Id is not valid.".to_string(),
            ))
        }
    };

    return Err(CustomError::new(
        StatusCode::NOT_FOUND,
        "Not found.".to_string(),
    ));
}

#[post("")]
async fn set_value(
    body: Json<SetRequestDTO>,
    _auth: AuthData,
    state: Data<AppState>,
) -> Result<HttpResponse, CustomError> {
    let body = body.into_inner();
    let mut data = state.db.lock().unwrap();

    let value_type = match body.value_type {
        Some(_v_type) => _v_type,
        None => ValueType::String,
    };

    match value_type {
        ValueType::String => {
            let result = data
                .set(body.value, body.lifetime)
                .to_custom_error("Could not do db operations")?;

            return Ok(HttpResponse::build(StatusCode::OK).json(DataResponseDTO {
                value: result.value,
                lifetime: result.lifetime,
                id: result.id,
            }));
        }
        ValueType::Integer32 => {
            let value: i32 = body
                .value
                .parse::<i32>()
                .to_custom_error("Could not parse to i32")?;

            let result = data
                .set(value, body.lifetime)
                .to_custom_error("Could not do db operations")?;

            return Ok(HttpResponse::build(StatusCode::OK).json(DataResponseDTO {
                value: result.value,
                lifetime: result.lifetime,
                id: result.id,
            }));
        }
        ValueType::Integer64 => {
            let value: i64 = body
                .value
                .parse::<i64>()
                .to_custom_error("Could not parse to i64")?;

            let result = data
                .set(value, body.lifetime)
                .to_custom_error("Could not do db operations")?;

            return Ok(HttpResponse::build(StatusCode::OK).json(DataResponseDTO {
                value: result.value,
                lifetime: result.lifetime,
                id: result.id,
            }));
        }
        ValueType::Integer128 => {
            let value: i128 = body
                .value
                .parse::<i128>()
                .to_custom_error("Could not parse to i128")?;

            let result = data
                .set(value, body.lifetime)
                .to_custom_error("Could not do db operations")?;

            return Ok(HttpResponse::build(StatusCode::OK).json(DataResponseDTO {
                value: result.value,
                lifetime: result.lifetime,
                id: result.id,
            }));
        }
        ValueType::Float32 => {
            let value: f32 = body
                .value
                .parse::<f32>()
                .to_custom_error("Could not parse to f32")?;
            let result = data
                .set(value, body.lifetime)
                .to_custom_error("Could not do db operations")?;

            return Ok(HttpResponse::build(StatusCode::OK).json(DataResponseDTO {
                value: result.value,
                lifetime: result.lifetime,
                id: result.id,
            }));
        }
        ValueType::Float64 => {
            let value: f64 = body
                .value
                .parse::<f64>()
                .to_custom_error("Could not parse to f64")?;

            let result = data
                .set(value, body.lifetime)
                .to_custom_error("Could not do db operations")?;

            return Ok(HttpResponse::build(StatusCode::OK).json(DataResponseDTO {
                value: result.value,
                lifetime: result.lifetime,
                id: result.id,
            }));
        }
    }
}

#[delete("")]
async fn delete_value(
    body: Json<DeleteRequestDTO>,
    _auth: AuthData,
    state: Data<AppState>,
) -> Result<HttpResponse, CustomError> {
    let body = body.into_inner();
    let mut data = state.db.lock().unwrap();
    let requested_type = body.id.split("#").collect::<Vec<&str>>();
    if requested_type.len() != 2 {
        return Err(CustomError::new(
            StatusCode::BAD_REQUEST,
            "Id is not valid".to_string(),
        ));
    }
    let requested_type = requested_type[1];
    match requested_type {
        "str" => {
            let value: Result<DbValue<String>, String> = data.delete(body.id);
            if let Ok(value) = value {
                return Ok(HttpResponse::build(StatusCode::OK).json(DataResponseDTO {
                    value: value.value,
                    lifetime: value.lifetime,
                    id: value.id,
                }));
            }
        }
        "i32" => {
            let value: Result<DbValue<i32>, String> = data.delete(body.id);
            if let Ok(value) = value {
                return Ok(HttpResponse::build(StatusCode::OK).json(DataResponseDTO {
                    value: value.value,
                    lifetime: value.lifetime,
                    id: value.id,
                }));
            }
        }
        "i64" => {
            let value: Result<DbValue<i64>, String> = data.delete(body.id);
            if let Ok(value) = value {
                return Ok(HttpResponse::build(StatusCode::OK).json(DataResponseDTO {
                    value: value.value,
                    lifetime: value.lifetime,
                    id: value.id,
                }));
            }
        }
        "i128" => {
            let value: Result<DbValue<i128>, String> = data.delete(body.id);
            if let Ok(value) = value {
                return Ok(HttpResponse::build(StatusCode::OK).json(DataResponseDTO {
                    value: value.value,
                    lifetime: value.lifetime,
                    id: value.id,
                }));
            }
        }
        "f32" => {
            let value: Result<DbValue<f32>, String> = data.delete(body.id);
            if let Ok(value) = value {
                return Ok(HttpResponse::build(StatusCode::OK).json(DataResponseDTO {
                    value: value.value,
                    lifetime: value.lifetime,
                    id: value.id,
                }));
            }
        }
        "f64" => {
            let value: Result<DbValue<f64>, String> = data.delete(body.id);
            if let Ok(value) = value {
                return Ok(HttpResponse::build(StatusCode::OK).json(DataResponseDTO {
                    value: value.value,
                    lifetime: value.lifetime,
                    id: value.id,
                }));
            }
        }
        _ => {
            return Err(CustomError::new(
                StatusCode::BAD_REQUEST,
                "Id is not valid.".to_string(),
            ))
        }
    };

    return Err(CustomError::new(
        StatusCode::NOT_FOUND,
        "Not found.".to_string(),
    ));
}

pub fn init_routes(config: &mut ServiceConfig) {
    config.service(
        scope("/data")
            .service(get_value)
            .service(set_value)
            .service(delete_value),
    );
}
