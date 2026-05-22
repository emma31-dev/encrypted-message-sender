-- Add migration script here
-- 1. Create a new table with the desired schema
CREATE TABLE users_new (
    id TEXT PRIMARY KEY,
    username TEXT UNIQUE NOT NULL,
    password_hash BLOB NOT NULL,          -- changed from TEXT to BLOB
    nonce BLOB NOT NULL,
    public_key TEXT,
    date_joined TEXT NOT NULL DEFAULT (datetime('now')),
    deleted_at TEXT
);

-- 2. Copy data from the old table
--    Convert password_hash from TEXT to BLOB (SQLite does this automatically,
--    but we explicitly cast to ensure correct type)
INSERT INTO users_new (id, username, password_hash, nonce, public_key, date_joined, deleted_at)
SELECT 
    id, 
    username, 
    CAST(password_hash AS BLOB),   -- convert to BLOB
    CAST(nonce AS BLOB),
    public_key, 
    date_joined, 
    deleted_at
FROM users;

-- 3. Drop the old table
DROP TABLE users;

-- 4. Rename the new table
ALTER TABLE users_new RENAME TO users;

-- 5. Recreate indexes (if any – usually on username)
CREATE UNIQUE INDEX idx_users_username ON users(username);