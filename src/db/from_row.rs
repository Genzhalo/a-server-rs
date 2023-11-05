use std::time::SystemTime;
use tokio_postgres::Row;
use crate::app::entities::user::{user_type::UserType, User};

pub fn base_user_from_row(row: &Row, key: &str) -> User {
    User {
        id: row.get::<&str, String>(format!("{key}_user_id").as_str()),
        email: row.get::<&str, String>(format!("{key}_email").as_str()),
        first_name: row.get::<&str, String>(format!("{key}_first_name").as_str()),
        last_name: row.get::<&str, String>(format!("{key}_last_name").as_str()),
        password_alg: String::from(""),
        password_hash: String::from(""),
        u_type: UserType::from_str(
            row.get::<&str, String>(format!("{key}_type").as_str())
                .as_str(),
        ),
        created_at: row
            .get::<&str, SystemTime>(format!("{key}_created_at").as_str())
            .into(),
        tokens: vec![],
    }
}