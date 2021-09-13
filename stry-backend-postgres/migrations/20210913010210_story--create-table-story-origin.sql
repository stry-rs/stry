CREATE TABLE story_story_origin (
    story_id    VARCHAR(8)          NOT NULL,
    origin_id   VARCHAR(8)          NOT NULL,

    level       story_tag_level     NOT NULL,

    created     TIMESTAMP WITHOUT TIME ZONE     NOT NULL,
    updated     TIMESTAMP WITHOUT TIME ZONE     NOT NULL,

    PRIMARY KEY (story_id, origin_id)
);
