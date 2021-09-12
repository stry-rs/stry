CREATE TABLE story_story_authors (
    story_id    VARCHAR(8)  NOT NULL,
    author_id   VARCHAR(8)  NOT NULL,

    PRIMARY KEY (story_id, author_id)
);
