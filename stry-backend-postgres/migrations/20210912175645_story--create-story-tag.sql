CREATE TABLE story_story_tag (
    story_id    VARCHAR(8)      NOT NULL,
    tag_id      VARCHAR(8)      NOT NULL,

    created     TIMESTAMP WITHOUT TIME ZONE     NOT NULL,
    updated     TIMESTAMP WITHOUT TIME ZONE     NOT NULL,

    PRIMARY KEY (story_id, tag_id)
);
