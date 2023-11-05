use axum::{http::Method, Router};
use app::entities::notification::Notification;
use db::DB;
use dotenv::dotenv;
use handlers::{auth, conversation, notification, user, ws, project};
use serde::Serialize;
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::broadcast;
use tower_http::cors::{Any, CorsLayer};

mod app;
mod db;
mod handlers;

#[derive(Debug, Clone, Serialize)]
pub enum Event {
    CreateNotification(Notification),
}
pub struct AppState {
    db: DB,
    tx: broadcast::Sender<Event>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db = DB::connect().await;

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_headers(Any)
        .allow_methods([
            Method::GET,
            Method::DELETE,
            Method::PATCH,
            Method::POST,
            Method::PUT,
            Method::OPTIONS,
        ]);

    let (tx, _rx) = broadcast::channel(100);
    let app_state = Arc::new(AppState { db, tx });

    let app = Router::new()
        .merge(auth::build_routes())
        .merge(conversation::build_routes())
        .merge(user::build_routes())
        .merge(ws::build_routes())
        .merge(notification::build_routes())
        .merge(project::build_routes())
        .layer(cors)
        .with_state(app_state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();


}
