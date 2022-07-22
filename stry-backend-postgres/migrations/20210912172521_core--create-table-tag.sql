CREATE TABLE IF NOT EXISTS core_tag (
    id              VARCHAR(8)  UNIQUE  NOT NULL    PRIMARY KEY,

    content         TEXT                NOT NULL,
    description     TEXT                NOT NULL,

    created         TIMESTAMP WITHOUT TIME ZONE     NOT NULL,
    updated         TIMESTAMP WITHOUT TIME ZONE     NOT NULL
);
