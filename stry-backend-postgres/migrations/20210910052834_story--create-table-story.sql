CREATE TABLE IF NOT EXISTS story_story (
    id          VARCHAR(8)  UNIQUE  NOT NULL    PRIMARY KEY,
    name        TEXT                NOT NULL,
    summary     TEXT                NOT NULL,
    rating      story_rating        NOT NULL,
    state       story_state         NOT NULL,

    created     TIMESTAMP WITHOUT TIME ZONE     NOT NULL,
    updated     TIMESTAMP WITHOUT TIME ZONE     NOT NULL
);
