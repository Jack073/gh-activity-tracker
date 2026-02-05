CREATE TABLE IF NOT EXISTS migrations
(
    id         int,
    applied_at timestamp with time zone,
    applied    boolean
)