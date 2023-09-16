CREATE TABLE user
(
    id         INTEGER PRIMARY KEY NOT NULL,
    login      TEXT                NOT NULL,
    yandex_id  TEXT                NOT NULL,
    created_at TIMESTAMP           NOT NULL
);