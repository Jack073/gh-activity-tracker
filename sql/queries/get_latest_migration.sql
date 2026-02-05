SELECT COALESCE(MAX(id), 0)
FROM migrations
WHERE applied;