use std::time::SystemTime;

use crate::app::{
    email::auth::AuthEvents,
    entities::user::{user_token::UserToken, user_type::UserType},
    traits::repositories::user::TUserRepositories,
    utils::{
        hash_pwd::{hash_pwd, verify_pwd},
        jwt::{ClaimType, JWT}, validate_params::validate,
    }, errors::BaseError,
};
use serde::Deserialize;
use validator::{Validate, ValidationError};

#[derive(Debug, Validate, Deserialize)]
pub struct CreateInputData {
    #[validate(length(min = 2, message = "The first name length should be min 2 symbols"))]
    #[serde(rename = "firstName")]
    first_name: String,
    #[validate(length(min = 2, message = "The last name length should be min 2 symbols"))]
    #[serde(rename = "lastName")]
    last_name: String,
    #[validate(email(message = "Email is invalid"))]
    email: String,
    #[validate(length(min = 6, message = "Password is invalid"))]
    password: String,
    #[validate(custom(function = "validate_user_role", message = "User role is invalid"))]
    #[serde(rename = "type")]
    role: String,
    #[validate(phone(message = "Phone is invalid"))]
    phone: Option<String>,
}

fn validate_user_role(role: &str) -> Result<(), ValidationError> {
    if [UserType::Client.to_string(), UserType::Vendor.to_string()].contains(&role.to_string()) {
        return Ok(());
    }
    Err(ValidationError::new(""))
}

#[derive(Debug, Validate, Deserialize)]
pub struct LoginInputData {
    #[validate(email(message = "Email is invalid"))]
    email: String,
    #[validate(length(min = 6, message = "Password is invalid"))]
    password: String,
}

#[derive(Debug, Validate, Deserialize)]
pub struct EmailInputData {
    #[validate(email(message = "Email is invalid"))]
    email: String,
}

#[derive(Debug, Validate, Deserialize)]
pub struct PasswordInputData {
    #[validate(length(min = 6, message = "Password is invalid"))]
    password: String,
}

pub struct AuthService<'a> {
    user_rep: &'a (dyn TUserRepositories + Send + Sync),
    events: AuthEvents,
    duration_of_send_email: usize,
}

impl<'a> AuthService<'a> {
    pub fn default(user_rep: &'a (dyn TUserRepositories + Send + Sync)) -> Self {
        Self {
            user_rep,
            events: AuthEvents::default(),
            duration_of_send_email: 600,
        }
    }

    pub async fn create(&self, signup_data: CreateInputData) -> Result<String, BaseError> {
        match validate(&signup_data) {
            Ok(_) => (),
            Err(e) => return Err(e),
        };
        let user = self.user_rep.find_by_email(&signup_data.email, false).await;

        if user.is_some() {
            return Err(BaseError::new("The email already using".to_string()));
        }

        let (password_alg, password_hash) = match hash_pwd(&signup_data.password) {
            Ok(res) => res,
            Err(e) => return Err(BaseError::new(e)),
        };

        let user_insert_result = self
            .user_rep
            .insert(
                &signup_data.first_name,
                &signup_data.last_name,
                signup_data.phone.as_deref(),
                &signup_data.email,
                &password_hash,
                &password_alg,
                &signup_data.role,
                false,
                true,
            )
            .await;

        let user_id = match user_insert_result {
            Ok(id) => id,
            Err(e) => return Err(BaseError::new(e.to_string())),
        };

        match self
            .verification_email_notify(&user_id, &signup_data.email)
            .await
        {
            Ok(_) => (),
            Err(e) => return Err(e),
        }

        let admins = self.user_rep.find(vec![UserType::Admin], None, None, None).await;
        let emails: Vec<&str> = admins.iter().map(|u| u.email.as_str()).collect();

        match self.events.on_create_user(&signup_data.email, emails).await {
            Ok(()) => Ok(user_id),
            Err(e) => Err(BaseError::new(e.to_string())),
        }
    }

    pub async fn login(&self, data: LoginInputData) -> Result<String, BaseError> {
        match validate(&data) {
            Ok(_) => (),
            Err(e) => return Err(e),
        };
        let user_result = self.user_rep.find_by_email(&data.email, false).await;

        let (user, user_email) = match user_result {
            Some(user) => user,
            None => return Err(BaseError::new("User not found".to_string())),
        };

        if !user_email.is_verified {
            return Err(BaseError::new("Email is not verified yet".to_string()));
        }

        if !verify_pwd(&user.password_hash, &data.password) {
            return Err(BaseError::new("Password is incorrect".to_string()));
        }

        let access_token = match JWT::default().access(&user) {
            Ok(token) => token,
            Err(err) => return Err(BaseError::new(err)),
        };

        let res = self
            .user_rep
            .upsert_user_token(&user.id, &access_token, "WEB");

        match res.await {
            Ok(_) => Ok(access_token),
            Err(err) => Err(BaseError::new(err)),
        }
    }

    pub async fn send_email_verification(&self, data: EmailInputData) -> Result<(), BaseError> {
        match validate(&data) {
            Ok(_) => (),
            Err(e) => return Err(e),
        };

        let user_result = self.user_rep.find_by_email(&data.email, true).await;

        let (user, user_email) = match user_result {
            Some(user) => user,
            None => return Err(BaseError::new("User not found".to_string())),
        };

        if user_email.is_verified {
            return Err(BaseError::new("Email is already verified".to_string()));
        }

        let user_token = user.tokens.iter().find(|t| t.used_for == "SEND_EMAIL");

        match self.check_can_send_email(user_token).await {
            Ok(_) => (),
            Err(err) => return Err(err),
        }

        self.verification_email_notify(&user.id, &data.email).await
    }

    pub async fn email_verify(&self, token: &str) -> Result<(), BaseError> {
        let email = match JWT::default().parse(token, Some(ClaimType::VerifyEmail)) {
            Ok(claim) => claim.sub,
            Err(e) => return Err(BaseError::new(e)),
        };

        let user_result = self.user_rep.find_by_email(&email, true).await;

        let (user, user_email) = match user_result {
            Some(user) => user,
            None => return Err(BaseError::new("User not found".to_string())),
        };

        if user_email.is_verified {
            return Err(BaseError::new("Email is already verified".to_string()));
        }

        match user.tokens.iter().find(|t| t.token == token) {
            Some(_) => (),
            None => return Err(BaseError::new("Token is expired".to_string())),
        }

        match self
            .user_rep
            .remove_user_tokens(&user.id, vec![token])
            .await
        {
            Ok(_) => (),
            Err(e) => return Err(BaseError::new(e.to_string())),
        }

        match self.user_rep.update_verify_email(&email, true).await {
            Ok(_) => (),
            Err(e) => return Err(BaseError::new(e.to_string())),
        };

        match self.events.on_email_verified(&user).await {
            Ok(_) => Ok(()),
            Err(e) => Err(BaseError::new(e.to_string())),
        }
    }

    pub async fn forgot_password(&self, data: EmailInputData) -> Result<(), BaseError> {
        match validate(&data) {
            Ok(_) => (),
            Err(e) => return Err(e),
        };
        let user_result = self.user_rep.find_by_email(&data.email, true).await;

        let (user, user_email) = match user_result {
            Some(user) => user,
            None => return Err(BaseError::new("User not found".to_string())),
        };

        if !user_email.is_verified {
            return Err(BaseError::new("Email is not verified yet".to_string()));
        }

        let user_token = user.tokens.iter().find(|t| t.used_for == "FORGOT_PASSWORD");

        match self.check_can_send_email(user_token).await {
            Ok(_) => (),
            Err(err) => return Err(err),
        }

        let code = match JWT::default().forgot_password(&user) {
            Ok(token) => token,
            Err(e) => return Err(BaseError::new(e)),
        };

        let result = self
            .user_rep
            .upsert_user_token(&user.id, &code, "FORGOT_PASSWORD")
            .await;

        if result.is_err() {
            return Err(BaseError::new(result.err().unwrap()));
        }

        match self.events.on_forgot_password(&data.email, &code).await {
            Ok(()) => Ok(()),
            Err(e) => Err(BaseError::new(e.to_string())),
        }
    }

    pub async fn reset_password(
        &self,
        token: &str,
        data: PasswordInputData,
    ) -> Result<(), BaseError> {
        match validate(&data) {
            Ok(_) => (),
            Err(e) => return Err(e),
        };

        let user_id = match JWT::default().parse(token, Some(ClaimType::ForgotPassword)) {
            Ok(claim) => claim.sub,
            Err(e) => return Err(BaseError::new(e)),
        };

        let user = match self.user_rep.find_by_id(&user_id, true).await {
            Some(res) => res,
            None => return Err(BaseError::new("User not found".to_string())),
        };

        match user.tokens.iter().find(|t| t.token == token) {
            Some(_) => (),
            None => return Err(BaseError::new("Token is expired".to_string())),
        }

        let (alg, hash) = match hash_pwd(&data.password) {
            Ok(res) => res,
            Err(e) => return Err(BaseError::new(e)),
        };

        match self
            .user_rep
            .remove_user_tokens(&user_id, vec![token])
            .await
        {
            Ok(_) => (),
            Err(e) => return Err(BaseError::new(e.to_string())),
        }

        match self.user_rep.update_password(&user_id, &alg, &hash).await {
            Ok(_) => Ok(()),
            Err(err) => Err(BaseError::new(err)),
        }
    }

    pub async fn revoke_token(&self, token: &str) -> Result<(), BaseError> {
        let user_id = match JWT::default().parse(token, None) {
            Ok(claim) => claim.sub,
            Err(e) => return Err(BaseError::new(e)),
        };

        match self
            .user_rep
            .remove_user_tokens(&user_id, vec![token])
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => return Err(BaseError::new(e.to_string())),
        }
    }

    async fn check_can_send_email(&self, user_token: Option<&UserToken>) -> Result<(), BaseError> {
        if user_token.is_none() {
            return Ok(());
        }

        let claims = match JWT::default().parse(&user_token.unwrap().token, None) {
            Ok(claims) => claims,
            Err(_) => return Ok(()),
        };

        let sec_duration = match SystemTime::now().duration_since(claims.iat) {
            Ok(duration) => duration.as_secs(),
            Err(_) => return Err(BaseError::new("Something wrong".to_string())),
        };

        if sec_duration < self.duration_of_send_email as u64 {
            let left_sec = (self.duration_of_send_email as u64).abs_diff(sec_duration);
            let min = left_sec / 60;
            let sec = (left_sec - (min * 60)) as i32;
            return Err(BaseError::new(format!(
                "This will be available through {} minutes {} seconds.",
                min, sec
            )));
        }

        Ok(())
    }

    async fn verification_email_notify(&self, user_id: &str, email: &str) -> Result<(), BaseError> {
        let code = match JWT::default().verify_email(&email) {
            Ok(token) => token,
            Err(e) => return Err(BaseError::new(e)),
        };

        let result = self
            .user_rep
            .upsert_user_token(&user_id, &code, "SEND_EMAIL")
            .await;

        if result.is_err() {
            return Err(BaseError::new(result.err().unwrap()));
        }

        match self.events.on_send_email_verify(&email, &code).await {
            Ok(()) => Ok(()),
            Err(e) => Err(BaseError::new(e.to_string())),
        }
    }

}
