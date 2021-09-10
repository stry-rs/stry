CREATE TABLE core_settings (
    key         TEXT    UNIQUE  NOT NULL,
    value       TEXT            NOT NULL,

    created     TIMESTAMP WITHOUT TIME ZONE     NOT NULL,
    updated     TIMESTAMP WITHOUT TIME ZONE     NOT NULL
);
