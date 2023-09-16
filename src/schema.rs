// @generated automatically by Diesel CLI.

diesel::table! {
    token (id) {
        id -> Integer,
        access_token -> Text,
        expires_in -> Timestamp,
        created_at -> Timestamp,
        user_id -> Integer,
    }
}

diesel::table! {
    user (id) {
        id -> Integer,
        login -> Text,
        yandex_id -> Text,
        created_at -> Timestamp,
    }
}

diesel::joinable!(token -> user (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    token,
    user,
);
