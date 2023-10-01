use crate::core::entities::user::{user_email::UserEmail, user_type::UserType, User};
use async_trait::async_trait;
#[async_trait]
pub trait TUserRepositories {
    async fn insert(
        &self,
        email: &str,
        p_hash: &str,
        p_alg: &str,
        u_type: &str,
        is_verify: bool,
        is_primary: bool,
    ) -> Result<String, String>;

    async fn find_by_email(&self, email: &str) -> Option<(User, UserEmail)>;
    async fn find_by_id(&self, id: &str) -> Option<User>;
    async fn find_by_type(&self, u_type: UserType) -> Vec<User>;

    async fn update_password(&self, user_id: &str, alg: &str, hash: &str) -> Result<bool, String>;

    async fn upsert_user_token(
        &self,
        user_id: &str,
        token: &str,
        used_for: &str,
    ) -> Result<bool, String>;

    async fn find_token_by(&self, user_id: &str, used_for: &str) -> Option<String>;

    async fn remove_user_tokens(&self, user_id: &str, tokens: Vec<&str>) -> Result<(), String>;

    async fn update_verify_email(&self, email: &str, is_verify: bool) -> Result<bool, String>;
}
