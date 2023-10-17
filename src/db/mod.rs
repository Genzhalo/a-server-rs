use crate::app::traits::repositories::{
    notification::TNotificationRepositories, user::TUserRepositories,
};
use std::{fs, sync::Arc};
use tokio_postgres::NoTls;

use self::{notification::NotificationRepository, user::UserRepository};

mod notification;
mod user;

pub struct DB {
    pub users: Box<dyn TUserRepositories + Sync + Send>,
    pub notifications: Box<dyn TNotificationRepositories + Sync + Send>,
}

impl DB {
    pub async fn connect() -> Self {
        let url = std::env::var("DATABASE_URL").expect("set DATABASE_URL env variable");

        let (client, connection) = tokio_postgres::connect(&url, NoTls)
            .await
            .expect("Database not created");

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        let path = std::env::var("DATABASE_SCHEMA_FILE_PATH")
            .expect("set DATABASE_SCHEMA_FILE_PATH env variable");

        let migration = fs::read_to_string(path).expect("Not read file");
        client.batch_execute(&migration).await.expect("Not init db");

        let arc_client = Arc::new(client);

        DB {
            users: Box::new(UserRepository::new(arc_client.clone())),
            notifications: Box::new(NotificationRepository::new(arc_client.clone())),
        }
    }
}
