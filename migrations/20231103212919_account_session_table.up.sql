-- Create account_sessions table

CREATE TABLE account_sessions (
  id text PRIMARY KEY NOT NULL,
  account_id TEXT NOT NULL,
  expires_at TEXT NOT NULL,
  active INTEGER NOT NULL,
  inserted_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,

  FOREIGN KEY(account_id) REFERENCES accounts(id)
);
