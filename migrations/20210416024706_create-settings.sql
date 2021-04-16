-- code: language=postgres

CREATE TABLE Settings (
    Key         TEXT    UNIQUE                  NOT NULL,
    Value       TEXT                            NOT NULL,

    Created     TIMESTAMP WITHOUT TIME ZONE     NOT NULL,
    Updated     TIMESTAMP WITHOUT TIME ZONE     NOT NULL
);
