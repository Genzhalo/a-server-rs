pub mod user_email;
pub mod user_token;
pub mod user_type;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use self::{user_token::UserToken, user_type::UserType};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    #[serde(rename = "type")]
    pub u_type: UserType,
    #[serde(skip_serializing)]
    pub password_alg: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    #[serde(skip_serializing)]
    pub tokens: Vec<UserToken>,
}
