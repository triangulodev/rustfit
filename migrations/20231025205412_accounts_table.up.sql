-- create accounts table

CREATE TABLE accounts (
  id text PRIMARY KEY NOT NULL,
  name text NOT NULL,
  email text NOT NULL UNIQUE,
  password_hash text NOT NULL,
  inserted_at text NOT NULL,
  updated_at text NOT NULL
);

