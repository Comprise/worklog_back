use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, 
Serialize, Deserialize)]
pub struct Claims {
    pub token_type: String,
    pub id: i32,
    pub login: String,
    pub exp: i64,
    pub iat: i64,
}

#[derive(Debug, Clone, 
Serialize, Deserialize)]
pub struct JWTPair {
    pub access: String,
    pub refresh: String,
}

#[derive(Debug, Clone,
Serialize, Deserialize)]
pub struct JwtRefresh {
    pub refresh: String,
}
pub enum JWTType {
    Access,
    Refresh,
}

