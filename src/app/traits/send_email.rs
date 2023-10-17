use async_trait::async_trait;

#[async_trait]
pub trait TEmail {
    async fn send(&self, to: Vec<&str>, subject: String, html: String) -> Result<(), String>;
}
