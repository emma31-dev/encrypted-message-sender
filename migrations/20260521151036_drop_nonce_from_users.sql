-- Add migration script here
-- SQLite doesn't support DROP COLUMN directly, so we recreate the table.
CREATE TABLE users_new (
    id TEXT PRIMARY KEY,
    username TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    public_key TEXT,
    date_joined TEXT NOT NULL DEFAULT (datetime('now')),
    deleted_at TEXT
);

INSERT INTO users_new (id, username, password_hash, public_key, date_joined, deleted_at)
SELECT id, username, password_hash, public_key, date_joined, deleted_at FROM users;

DROP TABLE users;
ALTER TABLE users_new RENAME TO users;