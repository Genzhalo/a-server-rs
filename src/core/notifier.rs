use super::{email::Email, entities::user::User, traits::send_email::TEmail};
pub struct Notifier {
    email: Box<dyn TEmail + Sync + Send>,
}

impl Notifier {
    pub fn default() -> Self {
        Self {
            email: Box::new(Email::default()),
        }
    }
    #[allow(dead_code)]
    pub fn new(email: Box<dyn TEmail + Sync + Send>) -> Self {
        Self { email }
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
        let url = format!("http://localhost:5173/auth/confirmation-email?token={code}");
        let html = format!(
            "<div>
                <h1> Verify your email address </h1> 
                <div>
                      <a style='text-decoration: none' href={url}> Verify </button>
                </div>
            </div>"
        );
        let _res = self
            .email
            .send(vec![&email], String::from("Verify Email"), html)
            .await;

        Ok(())
    }

    pub async fn on_forgot_password(&self, email: &str, code: &str) -> Result<(), String> {
        println!("on_send_email_verify: email: {}, code: {}", email, code);
        let url = format!("http://localhost:5173/auth/reset-password?token={code}");
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
