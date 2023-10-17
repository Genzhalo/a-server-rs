use crate::app::entities::user::User;
use chrono::Utc;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::{fmt, time::SystemTime};

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ClaimType {
    Refresh,
    Login,
    VerifyEmail,
    ForgotPassword,
}

impl fmt::Display for ClaimType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ClaimType::Refresh => write!(f, "Refresh"),
            ClaimType::Login => write!(f, "Login"),
            ClaimType::VerifyEmail => write!(f, "VerifyEmail"),
            ClaimType::ForgotPassword => write!(f, "ForgotPassword"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub user_type: Option<String>,
    pub claim_type: String,
    pub exp: usize,
    pub iat: SystemTime,
}

pub struct JWT {
    secret: String,
}

impl JWT {
    pub fn default() -> Self {
        let secret = std::env::var("JWT_SECRET_KEY").expect("set JWT_SECRET_KEY env variable");
        Self { secret }
    }

    pub fn verify_email(&self, email: &str) -> Result<String, String> {
        let claims = Claims {
            sub: email.to_string(),
            user_type: None,
            claim_type: ClaimType::VerifyEmail.to_string(),
            exp: self.get_expiration(1),
            iat: SystemTime::now(),
        };
        self.create(&claims)
    }

    pub fn forgot_password(&self, user: &User) -> Result<String, String> {
        let claims = Claims {
            sub: user.id.to_string(),
            user_type: Some(user.u_type.to_string()),
            claim_type: ClaimType::ForgotPassword.to_string(),
            exp: self.get_expiration(1),
            iat: SystemTime::now(),
        };
        self.create(&claims)
    }

    pub fn access(&self, user: &User) -> Result<String, String> {
        let claims = Claims {
            sub: user.id.to_owned(),
            user_type: Some(user.u_type.to_string()),
            claim_type: ClaimType::Login.to_string(),
            exp: self.get_expiration(7),
            iat: SystemTime::now(),
        };
        self.create(&claims)
    }

    pub fn parse(&self, token: &str, claim_type: Option<ClaimType>) -> Result<Claims, String> {
        let token_message = decode::<Claims>(
            &token,
            &DecodingKey::from_secret(self.secret.as_ref()),
            &Validation::new(Algorithm::HS256),
        );

        let claims = match token_message {
            Ok(data) => data.claims,
            Err(err) => return Err(err.to_string()),
        };

        if claim_type.is_none() || claims.claim_type == claim_type.unwrap().to_string() {
            Ok(claims)
        } else {
            Err("Token is not valid".to_string())
        }
    }

    fn create(&self, claims: &Claims) -> Result<String, String> {
        let token_res = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_ref()),
        );

        match token_res {
            Ok(token) => Ok(token),
            Err(err) => Err(err.to_string()),
        }
    }

    fn get_expiration(&self, days: u8) -> usize {
        Utc::now()
            .checked_add_signed(chrono::Duration::days(days as i64))
            .expect("valid timestamp")
            .timestamp() as usize
    }
}
