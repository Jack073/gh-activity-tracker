CREATE TABLE IF NOT EXISTS migrations
(
    id         BIGINT,
    applied_at TIMESTAMP WITH TIME ZONE,
    applied    BOOLEAN
)