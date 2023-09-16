use serde::{Deserialize, Serialize};

#[derive(
Debug, Clone,
Serialize, Deserialize)]
pub struct AuthUrl {
    pub auth_url: String,
}

#[derive(
Debug, Clone,
Serialize, Deserialize)]
pub struct AuthCallback {
    pub code: String,
}