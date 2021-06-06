CREATE TABLE User (
    Email       TEXT    UNIQUE  NOT NULL,
    Name        TEXT            NOT NULL,
    Biography   TEXT            NOT NULL,
    Hash        TEXT            NOT NULL,
    Settings    JSONB           NOT NULL,

    Created     TIMESTAMP WITHOUT TIME ZONE     NOT NULL,
    Updated     TIMESTAMP WITHOUT TIME ZONE     NOT NULL
);
