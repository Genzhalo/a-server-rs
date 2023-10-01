pub mod user_email;
pub mod user_type;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use self::user_type::UserType;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
    pub id: String,
    pub email: String,
    pub u_type: UserType,
    #[serde(skip_serializing)]
    pub password_alg: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
}
