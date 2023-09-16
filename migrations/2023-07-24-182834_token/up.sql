CREATE TABLE token
(
    id           INTEGER PRIMARY KEY                            NOT NULL,
    access_token TEXT                                           NOT NULL,
    expires_in   TIMESTAMP                                      NOT NULL,
    created_at   TIMESTAMP                                      NOT NULL,
    user_id      INTEGER REFERENCES user (id) ON DELETE CASCADE NOT NULL UNIQUE
);