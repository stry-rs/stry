CREATE TABLE IF NOT EXISTS core_user (
    id          VARCHAR(8)  UNIQUE  NOT NULL    PRIMARY KEY,
    email       TEXT        UNIQUE  NOT NULL,
    name        TEXT                NOT NULL,
    biography   TEXT                NOT NULL,
    hash        TEXT                NOT NULL,
    settings    JSONB               NOT NULL,

    created     TIMESTAMP WITHOUT TIME ZONE     NOT NULL,
    updated     TIMESTAMP WITHOUT TIME ZONE     NOT NULL
);
