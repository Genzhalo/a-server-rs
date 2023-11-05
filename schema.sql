CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS users (
  id                VARCHAR(36) NOT NULL PRIMARY KEY DEFAULT uuid_generate_v4(),
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

CREATE TABLE IF NOT EXISTS project (
  id                      VARCHAR(36) NOT NULL PRIMARY KEY DEFAULT uuid_generate_v4(),
  name                    VARCHAR(64) NOT NULL,
  city                    VARCHAR(64) NOT NULL,
  status                  VARCHAR(8) NOT NULL,
  street                  VARCHAR(255),
  zip_code                VARCHAR(16),
  floor                   VARCHAR(36),
  description             VARCHAR,
  building_type           VARCHAR(64) NOT NULL,
  user_id                 VARCHAR(36) NOT NULL,
  is_save_carbon          BOOLEAN NOT NULL,
  appropriate_status      VARCHAR(64) NOT NULL,
  budget_range            VARCHAR(32) NOT NULL,
  square_range            VARCHAR(32) NOT NULL,
  commercial_work         VARCHAR(64) NOT NULL,
  has_financing_secured   BOOLEAN NOT NULL, 
  completion_date         timestamp NOT NULL,
  architectural_services  text[],
  created_at              timestamp NOT NULL DEFAULT NOW(),
  updated_at              timestamp DEFAULT NOW(),

  CONSTRAINT fk_project_creator
    FOREIGN KEY(user_id) 
      REFERENCES users(id)
        ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_project_name ON project (name);
CREATE INDEX IF NOT EXISTS idx_project_status ON project (status);