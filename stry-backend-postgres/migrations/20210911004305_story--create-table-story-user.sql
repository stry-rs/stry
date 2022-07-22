CREATE TABLE IF NOT EXISTS story_story_user (
    story_id        VARCHAR(8)                  NOT NULL,
    user_id         VARCHAR(8)                  NOT NULL,

    relationship    story_user_relationship     NOT NULL,

    created         TIMESTAMP WITHOUT TIME ZONE     NOT NULL,
    updated         TIMESTAMP WITHOUT TIME ZONE     NOT NULL,

    PRIMARY KEY (story_id, user_id)
);
