SELECT
    s.id as "id: _",
    s.name,
    s.summary,
    s.rating as "rating: _",
    s.state as "state: _",
    s.created as "created: _",
    s.updated as "updated: _"
FROM
    story_story s
WHERE
    $1 = s.id
ORDER BY
    created DESC
LIMIT
    $2;
