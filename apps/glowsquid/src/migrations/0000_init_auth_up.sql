CREATE TABLE auth (
    id TEXT PRIMARY KEY,
    -- minecraft username
    username VARCHAR(16) NOT NULL,
    -- determines if all access tokens are encrypted and require a password to decrypt
    encrypted_token BOOLEAN NOT NULL,
    access_token TEXT NOT NULL,
    ms_access_token TEXT NOT NULL,
    ms_refresh_token TEXT NOT NULL,
    -- ISO 8601 date format
    expires_at TEXT NOT NULL,
);