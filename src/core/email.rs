use async_trait::async_trait;
use mailgun_rs::{EmailAddress, Mailgun, Message};

use super::traits::send_email::TEmail;

pub struct Email {
    key: String,
    domain: String,
}

impl Email {
    pub fn default() -> Self {
        let key = std::env::var("MAILGUN_KEY").expect("set SECRET_KEY env variable");
        let domain = std::env::var("MAILGUN_DOMAIN").expect("set SECRET_KEY env variable");
        Email { key, domain }
    }
}
#[async_trait]
impl TEmail for Email {
    async fn send(&self, to: Vec<&str>, subject: String, html: String) -> Result<(), String> {
        let recipients: Vec<EmailAddress> = to
            .into_iter()
            .map(|email| EmailAddress::address(email))
            .collect();

        let msg = Message {
            to: recipients,
            html,
            subject,
            ..Default::default()
        };

        let client = Mailgun {
            api_key: self.key.to_string(),
            domain: self.domain.to_string(),
            message: msg,
        };

        let sender = EmailAddress::address(format!("noreply@{}", self.domain).as_str());

        let res = client
            .async_send(mailgun_rs::MailgunRegion::US, &sender)
            .await;

        match res {
            Ok(_) => Ok(()),
            Err(err) => Err(err.to_string()),
        }
    }
}
