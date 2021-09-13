SELECT
    tag_id as id
FROM
    story_story_tag
WHERE
    story_id = $1;