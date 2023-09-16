use actix_web::{get, post, web, HttpResponse, Responder, Result, error};
use oauth2::basic::BasicClient;
use oauth2::{CsrfToken, Scope, TokenResponse};
use crate::config::Config;
use crate::db::DbPool;
use crate::jwt::{jwt_pair, verify_token};
use crate::models::{AuthUrl, AuthCallback, User, YandexToken, NewYandexToken, JwtRefresh, JWTType};
use crate::oauth::{get_token_result, get_user_info};
use jsonwebtoken::errors::{ErrorKind, Error};

#[get("/auth")]
async fn auth(
    oauth_client: web::Data<BasicClient>
) -> Result<impl Responder> {
    let (auth_url, _) = oauth_client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("login:info".to_string()))
        .add_scope(Scope::new("tracker:read".to_string()))
        .url();
    let auth_url = AuthUrl {
        auth_url: auth_url.to_string()
    };
    Ok(HttpResponse::Ok().json(auth_url))
}

#[post("/auth/refresh")]
async fn refresh(
    pool: web::Data<DbPool>,
    config: web::Data<Config>,
    data: web::Json<JwtRefresh>
) -> Result<impl Responder> {
    let pool = pool.into_inner();
    let config = config.into_inner();
    let data = data.into_inner();

    let user_id = verify_token(&config.secret_key, &data.refresh, &JWTType::Refresh)
        .map_err(|e| error::ErrorBadRequest(e.to_string()))?;

    let pool_clone = pool.clone();
    let user = web::block(move || {
        let mut conn = pool_clone.get()?;
        User::find_by_id(&mut conn, &user_id)
    })
        .await?
        .map_err(|e| error::ErrorBadRequest(e.to_string()))?;

    match user {
        None => Err(error::ErrorBadRequest(Error::from(ErrorKind::InvalidToken).to_string())),
        Some(user) => {
            let jwt_pair = jwt_pair(&config.clone(), &user.id, &user.login)
                .map_err(|e| error::ErrorBadRequest(e.to_string()))?;
            Ok(HttpResponse::Ok().json(jwt_pair))
        }
    }
}

#[get("/auth/callback")]
async fn auth_callback(
    pool: web::Data<DbPool>,
    config: web::Data<Config>,
    oauth_client: web::Data<BasicClient>,
    auth_callback: web::Query<AuthCallback>,
) -> Result<impl Responder> {
    let auth_callback = auth_callback.into_inner();
    let config = config.into_inner();
    let oauth_client = oauth_client.into_inner();
    let pool = pool.into_inner();

    let token_result = get_token_result(&oauth_client, &auth_callback.code)
        .await
        .map_err(|e| error::ErrorInternalServerError(e.to_string()))?;

    let user_data = get_user_info(&token_result.access_token().secret())
        .await
        .map_err(|e| error::ErrorInternalServerError(e.to_string()))?;

    let yandex_id = user_data.id.clone();
    let pool_clone = pool.clone();
    let user = web::block(move || {
        let mut conn = pool_clone.get()?;
        User::find_by_yandex_id(&mut conn, &yandex_id)
    })
        .await?
        .map_err(|e| error::ErrorInternalServerError(e.to_string()))?;

    let pool_clone = pool.clone();
    let user = match user {
        None => {
            web::block(move || {
                let mut conn = pool_clone.get()?;
                User::create(&mut conn, &user_data)
            })
                .await?
                .map_err(|e| error::ErrorInternalServerError(e.to_string()))?
        }
        Some(user) => user
    };

    let token_data = NewYandexToken::from((&token_result, &user.id));
    let token_data_clone = token_data.clone();
    let pool_clone = pool.clone();
    let token = web::block(move || {
        let mut conn = pool_clone.get()?;
        YandexToken::update(&mut conn, &token_data_clone)
    })
        .await?
        .map_err(|e| error::ErrorInternalServerError(e.to_string()))?;

    match token {
        None => {
            web::block(move || {
                let mut conn = pool.get()?;
                YandexToken::create(&mut conn, &token_data)
            })
                .await?
                .map_err(|e| error::ErrorInternalServerError(e.to_string()))?
        }
        Some(token) => token
    };

    let jwt_pair = jwt_pair(&config.clone(), &user.id, &user.login)
        .map_err(|e| error::ErrorInternalServerError(e.to_string()))?;
    Ok(HttpResponse::Ok().json(jwt_pair))
}