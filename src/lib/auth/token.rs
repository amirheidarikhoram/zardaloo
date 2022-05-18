use crate::lib::error::CustomError;
use crate::statics::{PASSWORD, USERNAME};
use actix_web::http::StatusCode;
use chrono::{DateTime, Duration, Utc};
use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use sha2::Sha256;
use std::collections::BTreeMap;
use std::env;

pub fn sign(username: String, password: String) -> Result<String, CustomError> {
    if username != *USERNAME || password != *PASSWORD {
        return Err(CustomError::new(
            StatusCode::UNAUTHORIZED,
            "Username or password is wrong".to_string(),
        ));
    }

    let utc_now = Utc::now();
    let secret = env::var("TOKEN_SECRET").expect("Could not get secret from env variables");
    let key: Hmac<Sha256> = Hmac::new_from_slice(secret.as_bytes()).unwrap();
    let mut claims = BTreeMap::new();

    claims.insert("username", username);
    claims.insert("signed_at", utc_now.to_rfc3339());
    claims.insert("valid_till", (utc_now + Duration::days(7)).to_rfc3339());

    let token = claims.sign_with_key(&key).unwrap();

    Ok(token)
}

pub fn verify(token: String) -> Result<bool, CustomError> {
    let secret = env::var("TOKEN_SECRET").expect("Could not get secret from env variables");
    let key: Hmac<Sha256> = Hmac::new_from_slice(secret.as_bytes()).unwrap();
    if let Ok(claims) = token.verify_with_key(&key) as Result<BTreeMap<String, String>, _> {
        // check if it's not expired
        let utc_now = Utc::now();
        let utc_expire_date = DateTime::parse_from_rfc3339(&claims["valid_till"][..])
            .unwrap()
            .with_timezone(&Utc);

        if utc_now < utc_expire_date {
            // it is still valid
            // lets get user model

            Ok(true)
        } else {
            // expired
            Err(CustomError::new(
                StatusCode::UNAUTHORIZED,
                "Token expired".to_string(),
            ))
        }
    } else {
        Err(CustomError::new(
            StatusCode::UNAUTHORIZED,
            "Token structure is not valid".to_string(),
        ))
    }
}
