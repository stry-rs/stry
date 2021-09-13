SELECT
    warning_id as id,
    level as "level: String"
FROM
    story_story_warning
WHERE
    story_id = $1;
