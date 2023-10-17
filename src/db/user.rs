use crate::app::{
    entities::user::{user_email::UserEmail, user_token::UserToken, user_type::UserType, User},
    traits::repositories::user::TUserRepositories,
};
use async_trait::async_trait;
use std::{sync::Arc, time::SystemTime};
use tokio_postgres::{Client, Row};

pub struct UserRepository {
    client: Arc<Client>,
}

impl User {
    fn from_rows(rows: &Vec<Row>) -> Self {
        User {
            id: rows[0].get::<&str, String>("id"),
            email: rows[0].get::<&str, String>("email"),
            first_name: rows[0].get::<&str, String>("first_name"),
            last_name: rows[0].get::<&str, String>("last_name"),
            password_alg: rows[0].get::<&str, String>("password_alg"),
            password_hash: rows[0].get::<&str, String>("password_hash"),
            u_type: UserType::from_str(rows[0].get::<&str, String>("type").as_str()),
            created_at: rows[0].get::<&str, SystemTime>("created_at").into(),
            tokens: rows
                .iter()
                .filter_map(|r| match r.try_get::<&str, String>("token") {
                    Ok(token) => Some(UserToken {
                        token: token,
                        used_for: r.get::<&str, String>("used_for"),
                    }),
                    Err(_) => None,
                })
                .collect(),
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
    pub fn new(client: Arc<Client>) -> Self {
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

    async fn find_by_email(&self, email: &str, with_tokens: bool) -> Option<(User, UserEmail)> {
        let statement = if with_tokens {
            "SELECT e.*, u.*, t.token, t.type as used_for FROM user_emails AS e
                JOIN users AS u ON u.id = e.user_id AND e.email = $1
                LEFT JOIN user_tokens AS t ON u.id = t.user_id;"
        } else {
            "SELECT e.*, u.* FROM user_emails AS e
                INNER JOIN users AS u ON u.id = e.user_id AND e.email = $1;"
        };

        let res = self.client.query(statement, &[&email]).await;

        match res {
            Ok(rows) => {
                if rows.len() > 0 {
                    Some((
                        User::from_rows(rows.as_ref()),
                        UserEmail::from_row(&rows[0]),
                    ))
                } else {
                    None
                }
            }
            Err(_) => None,
        }
    }

    async fn find_by_id(&self, id: &str, with_tokens: bool) -> Option<User> {
        let statement = if with_tokens {
            "SELECT e.*, u.*, t.token, t.type as used_for FROM users AS u
                JOIN user_emails AS e ON u.id = $1 AND e.user_id = $1 AND e.is_primary = true
                LEFT JOIN user_tokens AS t ON u.id = t.user_id;"
        } else {
            "SELECT e.*, u.* FROM users AS u
                INNER JOIN user_emails AS e ON u.id = $1 AND e.user_id = $1 AND e.is_primary = true;"
        };

        let res = self.client.query(statement, &[&id]).await;
        match res {
            Ok(rows) => {
                if rows.len() > 0 {
                    Some(User::from_rows(&rows))
                } else {
                    None
                }
            }
            Err(_err) => None,
        }
    }

    async fn find_by_type(&self, u_type: UserType) -> Vec<User> {
        let statement =
            format!("SELECT * FROM users JOIN user_emails ON id = user_id AND type = $1");
        let res = self.client.query(&statement, &[&u_type.to_string()]).await;
        match res {
            Ok(rows) => rows
                .into_iter()
                .map(|row| User::from_rows(&vec![row]))
                .collect(),
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
