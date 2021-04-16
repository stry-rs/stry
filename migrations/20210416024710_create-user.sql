-- code: language=postgres

CREATE TABLE User (
    Key     TEXT    UNIQUE  NOT NULL,
    Value   TEXT            NOT NULL
);
