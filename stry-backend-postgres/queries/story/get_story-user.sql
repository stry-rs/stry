SELECT
    user_id as id
FROM
    story_story_user
WHERE
    story_id = $1
    AND
    relationship::text = $2;
