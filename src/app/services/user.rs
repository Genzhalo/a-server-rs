use crate::app::{
    entities::user::User,
    errors::BaseError,
    traits::repositories::user::TUserRepositories,
    utils::jwt::{ClaimType, JWT},
};

pub struct UserService<'a> {
    user_rep: &'a (dyn TUserRepositories + Send + Sync),
}

impl<'a> UserService<'a> {
    pub fn new(user_rep: &'a (dyn TUserRepositories + Send + Sync)) -> Self {
        Self { user_rep }
    }

    pub async fn get_current_user(&self, token: &str) -> Result<User, BaseError> {
        let id = match self.id_from_token(token) {
            Ok(id) => id,
            Err(e) => return Err(e),
        };

        let user = match self.user_rep.find_by_id(id.as_str(), true).await {
            Some(user) => user,
            None => return Err(BaseError::new("User not found".to_string())),
        };

        match user.tokens.iter().find(|t| t.token == token) {
            Some(_) => Ok(user),
            None => Err(BaseError::new("Token is expired".to_string())),
        }
    }

    fn id_from_token(&self, token: &str) -> Result<String, BaseError> {
        match JWT::default().parse(token, Some(ClaimType::Login)) {
            Ok(claim) => Ok(claim.sub),
            Err(e) => return Err(BaseError::new(e)),
        }
    }
}
