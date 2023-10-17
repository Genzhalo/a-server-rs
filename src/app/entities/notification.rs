use super::user::User;
use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Notification {
    pub id: i32,
    pub content: String,
    pub path: String,
    pub is_read: bool,
    pub is_delete: bool,
    pub sender: User,
    pub receiver: User,
    pub created_at: DateTime<Utc>,
}
