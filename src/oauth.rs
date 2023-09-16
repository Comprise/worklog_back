use std::sync::Arc;
use oauth2::basic::{BasicClient, BasicErrorResponse, BasicTokenResponse};
use oauth2::{AuthorizationCode, AuthUrl, ClientId, ClientSecret, RedirectUrl, RequestTokenError, TokenUrl};
use oauth2::reqwest::{async_http_client, Error};
use crate::Config;
use crate::models::UserData;

pub fn initialize_oauth_client(config: &Config) -> BasicClient {
    BasicClient::new(
        ClientId::new(config.client_id.to_string()),
        Some(ClientSecret::new(config.client_secret.to_string())),
        AuthUrl::new("https://oauth.yandex.com/authorize".to_string())
            .expect("Invalid authorization endpoint URL"),
        Some(TokenUrl::new("https://oauth.yandex.ru/token".to_string())
            .expect("Invalid token endpoint URL")))
        .set_redirect_uri(
            RedirectUrl::new(format!("{}/auth/callback", config.front_url))
                .expect("Invalid redirect endpoint URL"))
}

pub async fn get_token_result(oauth_client: &Arc<BasicClient>, code: &str)
    -> Result<BasicTokenResponse, RequestTokenError<Error<reqwest::Error>, BasicErrorResponse>> {
    oauth_client
        .exchange_code(AuthorizationCode::new(code.to_string()))
        .request_async(async_http_client)
        .await
}

pub async fn get_user_info(token_result: &str)
    -> Result<UserData, reqwest::Error> {
    let client = reqwest::Client::new();
    client.get("https://login.yandex.ru/info")
        .bearer_auth(token_result)
        .send()
        .await?
        .json()
        .await
}