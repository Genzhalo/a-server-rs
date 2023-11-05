use std::{sync::Arc, time::SystemTime};
use async_trait::async_trait;
use tokio_postgres::{Client, Row};
use crate::app::{entities::notification::Notification, traits::repositories::notification::TNotificationRepositories};
use super::from_row::base_user_from_row;

const S_USER_FIELDS: &str = "s_emails.email AS sender_email, 
    s.id AS sender_user_id, 
    s.first_name AS sender_first_name, 
    s.last_name AS sender_last_name, 
    s.created_at AS sender_created_at, 
    s.type AS sender_type";

const R_USER_FIELDS: &str = "r_emails.email AS receiver_email, 
    r.id AS receiver_user_id, 
    r.first_name AS receiver_first_name, 
    r.last_name AS receiver_last_name, 
    r.created_at AS receiver_created_at,
    r.type AS receiver_type";


impl Notification {
    fn from_row(row: &Row) -> Self {
        Notification {
            id: row.get::<&str, i32>("id"),
            content: row.get::<&str, String>("content"),
            path: row.get::<&str, String>("path"),
            is_delete: row.get::<&str, bool>("is_delete"),
            is_read: row.get::<&str, bool>("is_read"),
            sender: base_user_from_row(row, "sender"),
            receiver: base_user_from_row(row, "receiver"),
            created_at: row
                .get::<&str, SystemTime>(format!("created_at").as_str())
                .into(),
        }
    }
}

pub struct NotificationRepository {
    client: Arc<Client>,
}

impl NotificationRepository {
    pub fn new(client: Arc<Client>) -> Self {
        Self { client }
    }
}

#[async_trait]
impl TNotificationRepositories for NotificationRepository {
    async fn insert(
        &self,
        content: &str,
        path: &str,
        is_read: bool,
        is_delete: bool,
        sender_id: &str,
        receiver_ids: Vec<&str>,
    ) -> Result<i32, String> {
        let statement = "
            WITH \"notification\" AS (
                INSERT INTO notifications (content, path, sender_id) VALUES ($1, $2, $3) RETURNING id
            ), n_users As (
                INSERT INTO notification_user (notification_id, is_read, is_delete, user_id)
                SELECT id, $4, $5, user_id
                FROM \"notification\", unnest($6::VARCHAR[]) as user_id
            )
            SELECT id FROM \"notification\";
        ";

        let res = self
            .client
            .query_one(
                statement,
                &[
                    &content,
                    &path,
                    &sender_id,
                    &is_read,
                    &is_delete,
                    &receiver_ids,
                ],
            )
            .await;

            match res {
            Ok(row) => Ok(row.get::<&str, i32>("id")),
            Err(err) => match err.as_db_error() {
                Some(err) => Err(err.message().to_string()),
                None => Err(err.to_string()),
            },
        }
    }

    async fn find_by_receiver(
        &self,
        receiver_id: &str,
        is_delete: bool,
        _is_read: Option<bool>,
    ) -> Vec<Notification> {
        let statement = format!("
            SELECT n.created_at, n.id, n.content, n.path, 
                nu.is_delete AS is_delete, nu.is_read AS is_read,  
                {S_USER_FIELDS}, {R_USER_FIELDS}
            FROM notifications AS n 
                JOIN notification_user AS nu ON nu.notification_id = n.id AND nu.user_id = $1 AND nu.is_delete = $2 
                JOIN users AS s ON s.id = n.sender_id                 
                JOIN user_emails AS s_emails ON s_emails.user_id = s.id AND s_emails.is_primary = true
                JOIN users AS r on nu.user_id = r.id
                JOIN user_emails AS r_emails ON r_emails.user_id = r.id AND r_emails.is_primary = true;");
        let res = self
            .client
            .query(&statement, &[&receiver_id, &is_delete])
            .await;
        match res {
            Ok(rows) => rows.iter().map(|row| Notification::from_row(row)).collect(),
            Err(_) => vec![],
        }
    }
    async fn find_by_id(&self, id: i32, receiver_id: &str) -> Option<Notification> {
        let statement = format!("
            SELECT  n.created_at, n.id, n.content, n.path, 
                nu.is_delete AS is_delete, nu.is_read AS is_read,  
                {S_USER_FIELDS}, {R_USER_FIELDS}
            FROM notifications AS n 
                JOIN users AS s ON s.id = n.sender_id AND n.id = $1
                JOIN user_emails AS s_emails ON s_emails.user_id = s.id AND s_emails.is_primary = true
                JOIN notification_user AS nu ON nu.notification_id = n.id AND nu.user_id = $2
                JOIN users AS r on nu.user_id = r.id
                JOIN user_emails AS r_emails ON r_emails.user_id = r.id AND r_emails.is_primary = true;");

        let res = self.client.query_one(&statement, &[&id, &receiver_id]).await;

        match res {
            Ok(row) => Some(Notification::from_row(&row)),
            Err(_) => None
        }
    }
    async fn set_read_by_id(&self, id: i32, receiver_id: &str) -> Result<bool, String> {
        let statement = "UPDATE notification_user SET is_read = true WHERE notification_id = $1 AND user_id = $2;";
        let res = self.client.execute(statement, &[&id, &receiver_id]).await;

        match res {
            Ok(row) => Ok(row != 0),
            Err(err) => match err.as_db_error() {
                Some(err) => Err(err.message().to_string()),
                None => Err(err.to_string()),
            },
        }
    }
    async fn set_read_by_receiver(&self, receiver_id: &str) -> Result<bool, String> {
        let statement = "UPDATE notification_user SET is_read = true WHERE user_id = $1;";
        let res = self.client.execute(statement, &[&receiver_id]).await;
        match res {
            Ok(row) => Ok(row != 0),
            Err(err) => match err.as_db_error() {
                Some(err) => Err(err.message().to_string()),
                None => Err(err.to_string()),
            },
        }
    }
    async fn set_delete_by_id(&self, id: i32, receiver_id: &str) -> Result<bool, String> {
        let statement = "UPDATE notification_user SET is_delete = true WHERE notification_id = $1 AND user_id = $2;";
        let res = self.client.execute(statement, &[&id, &receiver_id]).await;
        match res {
            Ok(row) => Ok(row != 0),
            Err(err) => match err.as_db_error() {
                Some(err) => Err(err.message().to_string()),
                None => Err(err.to_string()),
            },
        }
    }
    async fn set_delete_by_receiver(&self, receiver_id: &str) -> Result<bool, String> {
        let statement = "UPDATE notification_user SET is_delete = true WHERE user_id = $1;";
        let res = self.client.execute(statement, &[&receiver_id]).await;
        match res {
            Ok(row) => Ok(row != 0),
            Err(err) => match err.as_db_error() {
                Some(err) => Err(err.message().to_string()),
                None => Err(err.to_string()),
            },
        }
    }
}
