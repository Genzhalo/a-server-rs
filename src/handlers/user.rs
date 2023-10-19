use crate::{app::services::user::{UserService, GetAllParams}, AppState};
use axum::{
    body::Body,
    extract::{State, Query},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use serde_json::json;
use std::sync::Arc;

use super::extra::extract::AuthData;

pub fn build_routes() -> Router<Arc<AppState>, Body> {
    Router::new().route("/users/current", get(get_current_user)).route("/users", get(get_all))
    
}

async fn get_current_user(State(state): State<Arc<AppState>>, auth: AuthData) -> Response {
    let service = UserService::new(state.db.users.as_ref(), &auth.token);

    match service.get_current_user().await {
        Ok(user) => (StatusCode::OK, Json(json!({ "data":  user }))).into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, Json(json!({ "data":  err }))).into_response(),
    }
}

async fn get_all(Query(params): Query<GetAllParams>, State(state): State<Arc<AppState>>, auth: AuthData)-> Response{
    let service = UserService::new(state.db.users.as_ref(), &auth.token);

    match service.get_all(params).await {
        Ok(user) => (StatusCode::OK, Json(json!({ "data":  user }))).into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, Json(json!({ "data":  err }))).into_response(),
    }
}