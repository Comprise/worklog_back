use std::sync::Arc;
use jsonwebtoken::{
    encode, decode, Header, Validation, EncodingKey, DecodingKey,
    errors::{Result, ErrorKind, Error},
};
use chrono::{Duration, Utc};
use crate::config::Config;
use crate::models::{JWTType, Claims, JWTPair};

pub fn encode_jwt(
    config: &Arc<Config>, id: &i32, login: &str, token_type: &JWTType)
    -> Result<String> {
    let (token_type, exp) = match token_type {
        JWTType::Access => ("access", config.jwt_access_exp),
        JWTType::Refresh => ("refresh", config.jwt_refresh_exp)
    };
    let secret = config.secret_key.as_bytes();
    let iat = Utc::now();
    let exp = iat + Duration::minutes(exp);
    let claim = Claims {
        token_type: token_type.to_string(),
        id: *id,
        login: login.to_string(),
        iat: iat.timestamp(),
        exp: exp.timestamp(),
    };
    encode(&Header::default(), &claim, &EncodingKey::from_secret(secret))
}

pub fn verify_token(secret_key: &str, token: &str, token_type: &JWTType) -> Result<i32> {
    let secret = secret_key.as_bytes();
    let token = decode::<Claims>(&token, &DecodingKey::from_secret(secret), &Validation::default())?;
    let token_type_check = match token_type {
        JWTType::Access => { token.claims.token_type == "access" }
        JWTType::Refresh => { token.claims.token_type == "refresh" }
    };
    match token_type_check {
        true => Ok(token.claims.id),
        false => Err(Error::from(ErrorKind::InvalidToken))
    }
}

pub fn jwt_pair(config: &Arc<Config>, id: &i32, login: &str) -> Result<JWTPair> {
    let access = encode_jwt(&config.clone(), &id, login,&JWTType::Access)?;
    let refresh = encode_jwt(&config.clone(), &id, login,&JWTType::Refresh)?;
    let jwt_pair = JWTPair {
        access,
        refresh,
    };
    Ok(jwt_pair)
}