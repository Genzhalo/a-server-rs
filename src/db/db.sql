CREATE TABLE IF NOT EXISTS users (
  id                VARCHAR(36) NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),
  password_alg      VARCHAR(8) NOT NULL,
  password_hash     VARCHAR(255) NOT NULL,
  type              VARCHAR(8) NOT NULL,
  created_at        timestamp NOT NULL DEFAULT NOW(),
  updated_at        timestamp DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_user_type ON users (type);

CREATE TABLE IF NOT EXISTS user_emails (
  user_id           VARCHAR(36) NOT NULL,
  email             VARCHAR(64) NOT NULL UNIQUE,
  is_verified       BOOLEAN NOT NULL,
  is_primary        BOOLEAN NOT NULL,

  CONSTRAINT fk_user_emails
    FOREIGN KEY(user_id) 
      REFERENCES users(id)
        ON DELETE CASCADE
);


CREATE TABLE IF NOT EXISTS user_tokens (
  id                SERIAL PRIMARY KEY,
  user_id           VARCHAR(36) NOT NULL,
  token             VARCHAR NOT NULL,
  type              VARCHAR(16) NOT NULL,

  CONSTRAINT fk_user_tokens
    FOREIGN KEY(user_id) 
      REFERENCES users(id)
        ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_user_token_token  ON user_tokens (token);
CREATE INDEX IF NOT EXISTS idx_user_token_type  ON user_tokens (type);