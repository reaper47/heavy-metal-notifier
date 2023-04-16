-- +goose Up
CREATE TABLE users
(
    id           INTEGER PRIMARY KEY,
    email        TEXT NOT NULL UNIQUE,
    is_confirmed INTEGER DEFAULT 0,
    created_at   INTEGER DEFAULT (unixepoch('now'))
);

-- +goose Down
DROP TABLE users;