use axum::{routing::post, Router};
use db::DB;
use dotenv::dotenv;
use handlers::auth;
use std::{net::SocketAddr, sync::Arc};
use tower_http::cors::{Any, CorsLayer};

mod core;
mod db;
mod handlers;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db_res = DB::connect().await;
    let db = Arc::new(db_res);
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_headers(Any)
        .allow_methods(Any);

    let app = Router::new()
        .route("/auth/signup", post(auth::sign_up))
        .route("/auth/signin", post(auth::sign_in))
        .route(
            "/auth/send-email-verification",
            post(auth::send_email_verification),
        )
        .route("/auth/email-verification", post(auth::email_verify))
        .route("/auth/forgot-password", post(auth::forgot_password))
        .route("/auth/reset-password", post(auth::reset_password))
        .route("/auth/revoke-token", post(auth::revoke_token))
        .layer(cors)
        .with_state(db);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
