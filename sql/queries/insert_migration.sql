INSERT INTO migrations (id, applied_at, applied)
VALUES ($1, NOW(), $2);