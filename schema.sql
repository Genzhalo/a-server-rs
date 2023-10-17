CREATE TABLE IF NOT EXISTS users (
  id                VARCHAR(36) NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),
  first_name        VARCHAR(64) NOT NULL,
  last_name         VARCHAR(64) NOT NULL,
  password_alg      VARCHAR(8) NOT NULL,
  password_hash     VARCHAR(255) NOT NULL,
  phone             VARCHAR(16),
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

CREATE INDEX IF NOT EXISTS idx_user_token_token ON user_tokens (token);
CREATE INDEX IF NOT EXISTS idx_user_token_type ON user_tokens (type);

CREATE TABLE IF NOT EXISTS notifications (
  id                    SERIAL PRIMARY KEY,
  content               VARCHAR NOT NULL,
  path                  VARCHAR(255) NOT NULL,
  sender_id             VARCHAR(36) NOT NULL,
  created_at            timestamp NOT NULL DEFAULT NOW(),

  CONSTRAINT fk_notifications_sender
    FOREIGN KEY(sender_id) 
      REFERENCES users(id)
        ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_notifications_sender ON notifications (sender_id);

CREATE TABLE IF NOT EXISTS notification_user(
  id                SERIAL PRIMARY KEY,
  notification_id   INT NOT NULL,  
  is_read           BOOLEAN NOT NULL,
  is_delete         BOOLEAN NOT NULL,
  user_id           VARCHAR(36),

  CONSTRAINT fk_notifications_user
    FOREIGN KEY(notification_id) 
      REFERENCES notifications(id)
        ON DELETE CASCADE,

  CONSTRAINT fk_notifications_receiver
    FOREIGN KEY(user_id) 
      REFERENCES users(id)
        ON DELETE SET NULL
);

CREATE INDEX IF NOT EXISTS idx_n_user_n_id ON notification_user (notification_id);
CREATE INDEX IF NOT EXISTS idx_n_user_is_delete ON notification_user (is_delete);
CREATE INDEX IF NOT EXISTS idx_n_user_user_id ON notification_user (user_id);
