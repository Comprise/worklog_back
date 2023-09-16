use chrono::{Duration, NaiveDateTime, Utc};
use diesel::prelude::*;
use oauth2::basic::BasicTokenResponse;
use oauth2::TokenResponse;
use serde::{Deserialize, Serialize};
use super::User;
use crate::schema::token;
use crate::db::DbError;

#[derive(
Debug, Clone,
Serialize, Deserialize,
Selectable, Queryable, Identifiable, Associations)]
#[diesel(belongs_to(User))]
#[diesel(table_name = token)]
pub struct YandexToken {
    pub id: i32,
    pub access_token: String,
    pub expires_in: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub user_id: i32,
}

#[derive(
Debug, Clone,
Serialize, Deserialize,
Insertable, AsChangeset)]
#[diesel(table_name = token)]
pub struct NewYandexToken {
    pub access_token: String,
    pub expires_in: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub user_id: i32,
}

impl YandexToken {
    pub fn update(
        conn: &mut SqliteConnection,
        token_data: &NewYandexToken) -> Result<Option<Self>, DbError> {
        let token = diesel::update(token::table)
            .filter(token::user_id.eq(token_data.user_id))
            .set(token_data)
            .get_result(conn)
            .optional()?;
        Ok(token)
    }

    pub fn create(
        conn: &mut SqliteConnection,
        token_data: &NewYandexToken) -> Result<Self, DbError> {
        let token = diesel::insert_into(token::table)
            .values(token_data)
            .get_result(conn)?;
        Ok(token)
    }

    pub fn get_by_user_id(
        conn: &mut SqliteConnection,
        user: &User) -> Result<Self, DbError> {
        let token = YandexToken::belonging_to(&user)
            .first(conn)?;
        Ok(token)
    }
}

impl From<(&BasicTokenResponse, &i32)> for NewYandexToken {
    fn from((token, user_id): (&BasicTokenResponse, &i32)) -> Self {
        let seconds = token.expires_in()
            .unwrap_or(std::time::Duration::from_secs(0))
            .as_secs();
        let duration = Duration::seconds(seconds as i64);
        NewYandexToken {
            access_token: token.access_token().secret().to_string(),
            expires_in: (Utc::now() + duration).naive_utc(),
            created_at: Utc::now().naive_utc(),
            user_id: *user_id,
        }
    }
}