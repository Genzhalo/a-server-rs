use crate::app::entities::notification::Notification;
use async_trait::async_trait;
#[async_trait]
pub trait TNotificationRepositories {
    async fn insert(
        &self,
        content: &str,
        path: &str,
        is_read: bool,
        is_delete: bool,
        sender_id: &str,
        receiver_ids: Vec<&str>,
    ) -> Result<i32, String>;

    async fn find_by_receiver(
        &self,
        receiver_id: &str,
        is_delete: bool,
        is_read: Option<bool>,
    ) -> Vec<Notification>;
    async fn find_by_id(&self, id: i32) -> Option<Notification>;
    async fn set_read_by_id(&self, id: i32) -> Result<bool, String>;
    async fn set_read_by_receiver(&self, receiver_id: &str) -> Result<bool, String>;
    async fn set_delete_by_id(&self, id: i32) -> Result<bool, String>;
    async fn set_delete_by_receiver(&self, receiver_id: &str) -> Result<bool, String>;
}
