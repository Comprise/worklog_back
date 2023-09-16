use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::user;
use crate::db::DbError;

#[derive(
Debug, Clone,
Serialize, Deserialize)]
pub struct UserData {
    pub id: String,
    pub login: String
}

#[derive(
Debug, Clone,
Serialize, Deserialize,
Selectable, Queryable, Identifiable)]
#[diesel(table_name = user)]
pub struct User {
    pub id: i32,
    pub login: String,
    pub yandex_id: String,
    pub created_at: NaiveDateTime
}

#[derive(
Debug, Clone,
Serialize, Deserialize,
Insertable)]
#[diesel(table_name = user)]
pub struct NewUser {
    pub login: String,
    pub yandex_id: String,
    pub created_at: NaiveDateTime
}

impl User {
    pub fn find_by_id(
        conn: &mut SqliteConnection,
        id: &i32) -> Result<Option<Self>, DbError> {
        let user = user::table
            .filter(user::id.eq(id))
            .first(conn)
            .optional()?;
        Ok(user)
    }
    
    pub fn find_by_yandex_id(
        conn: &mut SqliteConnection,
        yandex_id: &str) -> Result<Option<Self>, DbError> {
        let user = user::table
            .filter(user::yandex_id.eq(yandex_id))
            .first(conn)
            .optional()?;
        Ok(user)
    }

    pub fn create(
        conn: &mut SqliteConnection,
        user_data: &UserData) -> Result<Self, DbError> {
        let new_user = NewUser::from(user_data);
        let user = diesel::insert_into(user::table)
            .values(new_user)
            .get_result(conn)?;
        Ok(user)
    }
}

impl From<&UserData> for NewUser {
    fn from(user: &UserData) -> Self {
        NewUser {
            login: user.login.to_string(),
            yandex_id: user.id.to_string(),
            created_at: Utc::now().naive_utc(),
        }
    }
}