use self::user::UserRepository;
use crate::core::traits::repositories::user::TUserRepositories;
use std::fs;
use tokio_postgres::NoTls;

mod user;

pub struct DB {
    pub user: Box<dyn TUserRepositories + Sync + Send>,
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

        let migration = fs::read_to_string("src/db/db.sql").expect("Not read file");
        client.batch_execute(&migration).await.expect("Not init db");

        DB {
            user: Box::new(UserRepository::new(client)),
        }
    }
}
