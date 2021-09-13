SELECT
    origin_id as id,
    level as "level: String"
FROM
    story_story_origin
WHERE
    story_id = $1;