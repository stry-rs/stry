CREATE TABLE IF NOT EXISTS story_story_warning (
    story_id    VARCHAR(8)          NOT NULL,
    warning_id  VARCHAR(8)          NOT NULL,

    level       story_tag_level     NOT NULL,

    created     TIMESTAMP WITHOUT TIME ZONE     NOT NULL,
    updated     TIMESTAMP WITHOUT TIME ZONE     NOT NULL,

    PRIMARY KEY (story_id, warning_id)
);
