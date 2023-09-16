use std::future::{ready, Ready};
use actix_web::{dev, error, FromRequest, HttpRequest, web};
use jsonwebtoken::{decode, DecodingKey, Validation};
use crate::config::Config;
use crate::models::Claims;

#[derive(Debug)]
pub struct AuthorizationService {
    pub user_id: i32,
}

impl FromRequest for AuthorizationService {
    type Error = error::Error;
    type Future = Ready<Result<AuthorizationService, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future {
        ready(req
            .headers()
            .get("Authorization")
            .ok_or_else(|| error::ErrorUnauthorized("Authorization header not found"))
            .and_then(|auth_header| auth_header.to_str()
                .map_err(|err| error::ErrorUnauthorized(err.to_string())))
            .and_then(|auth_header| {
                let words = auth_header.split(" ").collect::<Vec<&str>>();
                let bearer = words.get(0).unwrap();
                if bearer.to_lowercase() != "bearer".to_string() {
                    return Err(error::ErrorUnauthorized("Bearer prefix not found"));
                }
                words.get(1).map(|token| *token)
                    .ok_or_else(|| error::ErrorUnauthorized("Token not found"))
            })
            .and_then(|token| {
                let config = req.app_data::<web::Data<Config>>()
                    .expect("Middleware authorization not config");
                let secret = config.secret_key.as_bytes();
                decode::<Claims>(
                    &token,
                    &DecodingKey::from_secret(secret),
                    &Validation::default())
                    .map_err(|err| error::ErrorUnauthorized(err.to_string()))
            })
            .and_then(|r| Ok(AuthorizationService {
                user_id: r.claims.id,
            }))
            .map_err(|err| error::ErrorUnauthorized(err.to_string()))
        )
    }
}