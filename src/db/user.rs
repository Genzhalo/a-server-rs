use crate::core::{
    entities::user::{user_email::UserEmail, user_type::UserType, User},
    traits::repositories::user::TUserRepositories,
};
use async_trait::async_trait;
use std::time::SystemTime;
use tokio_postgres::{Client, Row};

pub struct UserRepository {
    client: Client,
}

impl User {
    fn from_row(row: &Row) -> Self {
        User {
            id: row.get::<&str, String>("id"),
            email: row.get::<&str, String>("email"),
            password_alg: row.get::<&str, String>("password_alg"),
            password_hash: row.get::<&str, String>("password_hash"),
            u_type: UserType::from_str(row.get::<&str, String>("type").as_str()),
            created_at: row.get::<&str, SystemTime>("created_at").into(),
        }
    }
}

impl UserEmail {
    fn from_row(row: &Row) -> Self {
        UserEmail {
            email: row.get::<&str, String>("email"),
            is_primary: row.get::<&str, bool>("is_primary"),
            is_verified: row.get::<&str, bool>("is_verified"),
        }
    }
}

impl UserRepository {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}

#[async_trait]
impl TUserRepositories for UserRepository {
    async fn insert(
        &self,
        first_name: &str,
        last_name: &str,
        phone: Option<&str>,
        email: &str,
        p_hash: &str,
        p_alg: &str,
        u_type: &str,
        is_verify: bool,
        is_primary: bool,
    ) -> Result<String, String> {
        let user_insert =
            "INSERT INTO users (password_alg, password_hash, type, first_name, last_name, phone) 
                VALUES ($1, $2, $3, $7, $8, $9) RETURNING *";

        let email_insert = "
            INSERT INTO user_emails (email, is_primary, is_verified, user_id) 
                VALUES ($4, $5, $6, (SELECT id FROM \"user\")) RETURNING user_id as id";

        let res = self
            .client
            .query_one(
                format!("WITH \"user\" AS ({}) {};", user_insert, email_insert).as_str(),
                &[
                    &p_alg,
                    &p_hash,
                    &u_type,
                    &email,
                    &is_primary,
                    &is_verify,
                    &first_name,
                    &last_name,
                    &phone,
                ],
            )
            .await;

        match res {
            Ok(row) => Ok(row.get::<&str, String>("id")),
            Err(err) => match err.as_db_error() {
                Some(err) => Err(err.message().to_string()),
                None => Err(err.to_string()),
            },
        }
    }

    async fn find_by_email(&self, email: &str) -> Option<(User, UserEmail)> {
        let res = self
            .client
            .query_one(
                "SELECT * FROM user_emails JOIN users ON users.id = user_id AND email = $1",
                &[&email],
            )
            .await;

        match res {
            Ok(row) => Some((User::from_row(&row), UserEmail::from_row(&row))),
            Err(_) => None,
        }
    }

    async fn find_by_id(&self, id: &str) -> Option<User> {
        let res = self
            .client
            .query_one(
                "SELECT * FROM users 
                                  JOIN user_emails ON id = $1 AND 
                                                      user_id = $1 AND 
                                                      is_primary = true",
                &[&id],
            )
            .await;

        match res {
            Ok(row) => Some(User::from_row(&row)),
            Err(_err) => None,
        }
    }

    async fn find_by_type(&self, u_type: UserType) -> Vec<User> {
        let statement = format!(
            "
            SELECT * FROM users JOIN user_emails ON id = user_id AND type = $1"
        );
        let res = self.client.query(&statement, &[&u_type.to_string()]).await;
        match res {
            Ok(rows) => rows.into_iter().map(|row| User::from_row(&row)).collect(),
            Err(_) => vec![],
        }
    }

    async fn update_password(&self, user_id: &str, alg: &str, hash: &str) -> Result<bool, String> {
        let res = self
            .client
            .execute(
                "UPDATE users SET password_alg = $2, password_hash = $3 WHERE id = $1;",
                &[&user_id, &alg, &hash],
            )
            .await;

        match res {
            Ok(row) => Ok(row != 0),
            Err(err) => match err.as_db_error() {
                Some(err) => Err(err.message().to_string()),
                None => Err(err.to_string()),
            },
        }
    }

    async fn update_verify_email(&self, email: &str, is_verify: bool) -> Result<bool, String> {
        let res = self
            .client
            .execute(
                "UPDATE user_emails SET is_verified = $2 WHERE email = $1;",
                &[&email, &is_verify],
            )
            .await;

        match res {
            Ok(row) => Ok(row != 0),
            Err(err) => match err.as_db_error() {
                Some(err) => Err(err.message().to_string()),
                None => Err(err.to_string()),
            },
        }
    }

    async fn upsert_user_token(
        &self,
        user_id: &str,
        token: &str,
        used_for: &str,
    ) -> Result<bool, String> {
        let res = self
            .client
            .execute(
                "
                    WITH upsert AS (UPDATE user_tokens SET token = $2 WHERE user_id = $1 AND type = $3 RETURNING *)
                    INSERT INTO user_tokens (user_id, token, type) SELECT $1,$2,$3 WHERE NOT EXISTS (SELECT * FROM upsert);",
                &[&user_id, &token, &used_for],
            )
            .await;

        match res {
            Ok(row) => Ok(row != 0),
            Err(err) => match err.as_db_error() {
                Some(err) => Err(err.message().to_string()),
                None => Err(err.to_string()),
            },
        }
    }

    async fn find_token_by(&self, user_id: &str, used_for: &str) -> Option<String> {
        let res = self
            .client
            .query_one(
                "SELECT * FROM user_tokens  
                        WHERE user_id = $1 AND type = $2;",
                &[&user_id, &used_for],
            )
            .await;

        match res {
            Ok(row) => Some(row.get::<&str, String>("token")),
            Err(_err) => None,
        }
    }

    async fn remove_user_tokens(&self, user_id: &str, tokens: Vec<&str>) -> Result<(), String> {
        let res = self
            .client
            .execute(
                "DELETE FROM user_tokens WHERE user_id = $1 AND token = any($2);",
                &[&user_id, &tokens],
            )
            .await;

        match res {
            Ok(_) => Ok(()),
            Err(err) => match err.as_db_error() {
                Some(err) => Err(err.message().to_string()),
                None => Err(err.to_string()),
            },
        }
    }
}
