use crate::AppState;
use axum::{
    body::Body,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, patch, post},
    Router,
};
use std::sync::Arc;

use super::extra::extract::AuthData;

pub fn build_routes() -> Router<Arc<AppState>, Body> {
    Router::new()
        .route("/conversations/:id/messages", post(create_message))
        .route("/conversations/:id/messages", get(get_messages))
        .route("/conversations/:id/add-users", patch(add_users))
        .route("/conversations/:id/remove-users", patch(remove_users))
}

async fn create_message(
    Path(_id): Path<String>,
    State(_state): State<Arc<AppState>>,
    _auth: AuthData,
) -> Response {
    StatusCode::NOT_IMPLEMENTED.into_response()
}

async fn get_messages(
    Path(_id): Path<String>,
    State(_state): State<Arc<AppState>>,
    _auth: AuthData,
) -> Response {
    StatusCode::NOT_IMPLEMENTED.into_response()
}

async fn add_users(
    Path(_id): Path<String>,
    State(_state): State<Arc<AppState>>,
    _auth: AuthData,
) -> Response {
    StatusCode::NOT_IMPLEMENTED.into_response()
}

async fn remove_users(
    Path(_id): Path<String>,
    State(_state): State<Arc<AppState>>,
    _auth: AuthData,
) -> Response {
    StatusCode::NOT_IMPLEMENTED.into_response()
}
