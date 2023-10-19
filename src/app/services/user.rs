use serde::Deserialize;
use validator::{Validate, ValidationError};

use crate::app::{
    entities::user::{User, user_type::UserType},
    errors::BaseError,
    traits::repositories::user::TUserRepositories, utils::{jwt::{ClaimType, JWT}, validate_params::validate},
};

#[derive(Debug, Validate, Deserialize)]
pub struct GetAllParams {
    #[validate(custom(function = "validate_user_role", message = "User type is invalid"))]
    #[serde(rename = "type")]
    u_type: Option<String>, 
    limit: Option<i64>, 
    skip: Option<i64>,
    search: Option<String>
}

fn validate_user_role(role: &str) -> Result<(), ValidationError> {
    if [UserType::Client.to_string(), UserType::Vendor.to_string(), UserType::Admin.to_string()].contains(&role.to_string()) {
        return Ok(());
    }
    Err(ValidationError::new(""))
}

pub struct UserService<'a> {
    user_rep: &'a (dyn TUserRepositories + Send + Sync),
    token: &'a str
}

impl<'a> UserService<'a> {
    pub fn new(user_rep: &'a (dyn TUserRepositories + Send + Sync), token: &'a str) -> Self {
        Self { user_rep, token }
    }

    pub async fn get_current_user(&self) -> Result<User, BaseError> {

        let id = match self.id_from_token(self.token) {
            Ok(id) => id,
            Err(e) => return Err(e),
        };

        let user = match self.user_rep.find_by_id(id.as_str(), true).await {
            Some(user) => user,
            None => return Err(BaseError::new("User not found".to_string())),
        };

        match user.tokens.iter().find(|t| t.token == self.token) {
            Some(_) => Ok(user),
            None => Err(BaseError::new("Token is expired".to_string())),
        }
    }

    pub async fn get_all(&self, params: GetAllParams) -> Result<Vec<User>, BaseError>{
        match validate(&params) {
            Ok(_) => (),
            Err(e) => return Err(e),
        };

        let user = match self.get_current_user().await {
            Ok(user) => user,
            Err(e) => return Err(e),
        };

        if user.u_type != UserType::Admin {
            return Err(BaseError::new("Forbidden".to_string()));
        }

        let types = match params.u_type {
            Some(value) => vec![UserType::from_str(&value)],
            None => vec![UserType::Client, UserType::Vendor, UserType::Admin]
        };
        
        let users = self.user_rep.find(types, params.search.as_deref(), params.limit, params.skip).await;
        
        Ok(users)
    }

    fn id_from_token(&self, token: &str) -> Result<String, BaseError> {
        match JWT::default().parse(token, Some(ClaimType::Login)) {
            Ok(claim) => Ok(claim.sub),
            Err(e) => return Err(BaseError::new(e)),
        }
    }
}
