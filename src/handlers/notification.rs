use super::extra::extract::AuthData;
use crate::{app::services::notification::NotificationService, AppState};
use axum::{
    body::Body,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{delete, get, patch},
    Json, Router,
};
use serde_json::json;
use std::sync::Arc;

pub fn build_routes() -> Router<Arc<AppState>, Body> {
    Router::new()
        .route("/notifications", get(get_notifications))
        .route("/notifications/delete", delete(del_for_current_user))
        .route("/notifications/read", patch(read_for_current_user))
        .route("/notifications/:id", get(get_notification))
        .route("/notifications/:id/read", patch(read_notification))
        .route("/notifications/:id/delete", delete(del_notification))
}

async fn get_notifications(State(state): State<Arc<AppState>>, auth: AuthData) -> Response {
    let service =
        NotificationService::new(state.db.users.as_ref(), state.db.notifications.as_ref(), &auth.token);
    match service.get_all_for_current_user().await {
        Ok(data) => (StatusCode::OK, Json(json!({ "data":  data }))).into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, Json(json!({ "data":  err }))).into_response(),
    }
}

async fn del_for_current_user(State(state): State<Arc<AppState>>, auth: AuthData) -> Response {
    let service =
        NotificationService::new(state.db.users.as_ref(), state.db.notifications.as_ref(),&auth.token);
    match service.delete_all_for_current_user().await {
        Ok(data) => (StatusCode::OK, Json(json!({ "data":  data }))).into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, Json(json!({ "data":  err }))).into_response(),
    }
}

async fn read_for_current_user(State(state): State<Arc<AppState>>, auth: AuthData) -> Response {
    let service =
        NotificationService::new(state.db.users.as_ref(), state.db.notifications.as_ref(), &auth.token);

    match service.read_all_for_current_user().await {
        Ok(data) => (StatusCode::OK, Json(json!({ "data":  data }))).into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, Json(json!({ "data":  err }))).into_response(),
    }
}

async fn get_notification(
    Path(id): Path<i32>,
    State(state): State<Arc<AppState>>,
    auth: AuthData,
) -> Response {
    let service =
        NotificationService::new(state.db.users.as_ref(), state.db.notifications.as_ref(), &auth.token);

    match service.get_by_id(id).await {
        Ok(data) => (StatusCode::OK, Json(json!({ "data":  data }))).into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, Json(json!({ "data":  err }))).into_response(),
    }
}

async fn read_notification(
    Path(id): Path<i32>,
    State(state): State<Arc<AppState>>,
    auth: AuthData,
) -> Response {
    let service =
        NotificationService::new(state.db.users.as_ref(), state.db.notifications.as_ref(), &auth.token);

    match service.read_by_id(id).await {
        Ok(data) => (StatusCode::OK, Json(json!({ "data":  data }))).into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, Json(json!({ "data":  err }))).into_response(),
    }
}

async fn del_notification(
    Path(id): Path<i32>,
    State(state): State<Arc<AppState>>,
    auth: AuthData,
) -> Response {
    let service =
        NotificationService::new(state.db.users.as_ref(), state.db.notifications.as_ref(), &auth.token);

    match service.delete_by_id(id).await {
        Ok(data) => (StatusCode::OK, Json(json!({ "data":  data }))).into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, Json(json!({ "data":  err }))).into_response(),
    }
}
