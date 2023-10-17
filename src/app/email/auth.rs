use crate::app::{entities::user::User, traits::send_email::TEmail};

use super::Email;

pub struct AuthEvents {
    client_url: String,
    email: Box<dyn TEmail + Sync + Send>,
}

impl AuthEvents {
    pub fn default() -> Self {
        let client_url = std::env::var("CLIENT_URL").expect("set CLIENT_URL env variable");
        Self {
            client_url,
            email: Box::new(Email::default()),
        }
    }

    pub async fn on_create_user(&self, email: &str, _admins: Vec<&str>) -> Result<(), String> {
        println!("on_create_user: user : {:?}", email);
        Ok(())
    }

    pub async fn on_email_verified(&self, user: &User) -> Result<(), String> {
        println!("on_email_verified: user: {:?}", user);
        Ok(())
    }

    pub async fn on_send_email_verify(&self, email: &str, code: &str) -> Result<(), String> {
        println!("on_send_email_verify: email: {}, code: {}", email, code);
        let url = format!("{}/auth/confirmation-email?token={code}", self.client_url);
        let html = format!(
            "<div>
                <div>
                    <p> Hello, </p>
                    <p> Thank you for signing up! Please click below to confirm your email. </p>
                    <p> The team </p>
                </div>
                <div>
                    <a style='text-decoration: none' href={url}> 
                        Click here to confirm your email address 
                    </a>
                </div>
            </div>"
        );
        let _res = self
            .email
            .send(vec![&email], String::from("Email Confirmation"), html)
            .await;

        Ok(())
    }

    pub async fn on_forgot_password(&self, email: &str, code: &str) -> Result<(), String> {
        println!("on_send_email_verify: email: {}, code: {}", email, code);
        let url = format!("{}/auth/reset-password?token={code}", self.client_url);
        let html = format!(
            "<div>
                <h1>Forgot Password </h1> 
                <div>
                      <a style='text-decoration: none' href={url}> Reset </button>
                </div>
            </div>"
        );
        let _res = self
            .email
            .send(vec![&email], String::from("Reset Password"), html)
            .await;

        Ok(())
    }
}
