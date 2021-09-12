SELECT
    u.id,
    u.name,
    u.created as "created: _",
    u.updated as "updated: _"
FROM
    core_user u
LEFT JOIN
    story_story_authors sa ON sa.author_id = u.id
WHERE
    sa.story_id = $1;
