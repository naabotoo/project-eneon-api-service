CREATE EXTENSION IF NOT EXISTS pgcrypto;

CREATE TABLE api_client_credentials (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  client_id TEXT NOT NULL,
  client_secret TEXT NOT NULL,
  is_active BOOLEAN NOT NULL DEFAULT FALSE,
  created_on TIMESTAMP NOT NULL DEFAULT NOW(),
  updated_on TIMESTAMP NOT NULL DEFAULT NOW()
);
ALTER TABLE api_client_credentials ADD CONSTRAINT api_client_credentials_client_id_unique UNIQUE (client_id);