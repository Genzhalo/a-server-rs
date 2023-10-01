use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserEmail {
    pub is_primary: bool,
    pub is_verified: bool,
    pub email: String,
}
