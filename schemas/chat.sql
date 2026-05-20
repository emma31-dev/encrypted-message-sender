-- Users table
CREATE TABLE users (
    id TEXT PRIMARY KEY,                     -- UUID stored as string, generated in Rust
    username TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    date_joined TEXT NOT NULL DEFAULT (datetime('now')),
    public_key TEXT NOT NULL,
    deleted_at TEXT
);

-- Chats table
CREATE TABLE chats (
    id TEXT PRIMARY KEY,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    deleted_at TEXT
);

-- Chat participants (many-to-many)
CREATE TABLE chat_participants (
    chat_id TEXT NOT NULL REFERENCES chats(id) ON DELETE CASCADE,
    user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    joined_at TEXT NOT NULL DEFAULT (datetime('now')),
    PRIMARY KEY (chat_id, user_id)
);

-- Messages table (with encryption support)
CREATE TABLE messages (
    id TEXT PRIMARY KEY,                     -- UUID generated in Rust
    chat_id TEXT NOT NULL REFERENCES chats(id) ON DELETE CASCADE,
    sender_id TEXT REFERENCES users(id) ON DELETE SET NULL,
    encrypted_content BLOB NOT NULL,         -- AES-GCM ciphertext (without tag)
    nonce BLOB NOT NULL,                     -- 12 bytes unique per message
    tag BLOB NOT NULL,                       -- 16 bytes authentication tag
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Indexes
CREATE INDEX idx_messages_chat_id ON messages(chat_id);
CREATE INDEX idx_messages_sender_id ON messages(sender_id);
CREATE INDEX idx_messages_created_at ON messages(created_at);
CREATE INDEX idx_chat_participants_user_id ON chat_participants(user_id);
CREATE INDEX idx_chat_participants_chat_id ON chat_participants(chat_id);