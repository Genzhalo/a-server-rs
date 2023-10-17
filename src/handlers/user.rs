use crate::{app::services::user::UserService, AppState};
use axum::{
    body::Body,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use serde_json::json;
use std::sync::Arc;

use super::extra::extract::AuthData;

pub fn build_routes() -> Router<Arc<AppState>, Body> {
    Router::new().route("/users/current", get(get_current_user))
}

async fn get_current_user(State(state): State<Arc<AppState>>, auth: AuthData) -> Response {
    let service = UserService::new(state.db.users.as_ref());

    match service.get_current_user(&auth.token).await {
        Ok(user) => (StatusCode::OK, Json(json!({ "data":  user }))).into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, Json(json!({ "data":  err }))).into_response(),
    }
}
