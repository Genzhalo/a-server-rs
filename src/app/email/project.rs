use crate::app::{entities::user::User, traits::send_email::TEmail};

use super::Email;

pub struct ProjectEvents {
    client_url: String,
    email: Box<dyn TEmail + Sync + Send>,
}

impl ProjectEvents {
    pub fn default() -> Self {
        let client_url = std::env::var("CLIENT_URL").expect("set CLIENT_URL env variable");
        Self {
            client_url,
            email: Box::new(Email::default()),
        }
    }

    pub async fn on_create_project(&self, user: User, title: &str, id: &str, admin_emails: Vec<&str>) -> Result<(), String> {
        let url = format!("{}/a/projects/{}", self.client_url, id);
        let user_name = format!("{} {}", user.first_name, user.last_name);
        let html = format!(
            "<div>
                <p> {user_name} created a new project {title}</p>
                <p>
                    <a style='text-decoration: none' href={url}> 
                        Review the project 
                    </a>
                </p>
            </div>", 
        );
        let _res = self
            .email
            .send(admin_emails, String::from("Project Created"), html)
            .await;

        Ok(())
    }

    
}
