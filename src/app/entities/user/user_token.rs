use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct UserToken {
    pub token: String,
    pub used_for: String,
}
