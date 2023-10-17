use std::sync::Arc;

use axum::{
    body::Body,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
    Json, Router,
};
use serde_json::json;

use crate::{
    app::services::auth::{
        AuthService, CreateInputData, EmailInputData, LoginInputData, PasswordInputData,
    },
    AppState,
};

use super::extra::{extract::AuthData, json_validate_rejection::JsonInput};

pub fn build_routes() -> Router<Arc<AppState>, Body> {
    Router::new()
        .route("/auth/signup", post(sign_up))
        .route("/auth/signin", post(sign_in))
        .route(
            "/auth/send-email-verification",
            post(send_email_verification),
        )
        .route("/auth/email-verification", post(email_verify))
        .route("/auth/forgot-password", post(forgot_password))
        .route("/auth/reset-password", post(reset_password))
        .route("/auth/revoke-token", post(revoke_token))
}

async fn sign_up(
    State(state): State<Arc<AppState>>,
    JsonInput(body): JsonInput<CreateInputData>,
) -> Response {
    let service = AuthService::default(state.db.users.as_ref());

    match service.create(body).await {
        Ok(id) => (StatusCode::OK, Json(json!({"data": { "id": id }}))).into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, Json(json!({ "data": err }))).into_response(),
    }
}

async fn sign_in(
    State(state): State<Arc<AppState>>,
    JsonInput(body): JsonInput<LoginInputData>,
) -> Response {
    let service = AuthService::default(state.db.users.as_ref());

    match service.login(body).await {
        Ok(token) => (StatusCode::OK, Json(json!({"data": token}))).into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, Json(json!({ "data":  err }))).into_response(),
    }
}

async fn email_verify(State(state): State<Arc<AppState>>, data: AuthData) -> Response {
    let service = AuthService::default(state.db.users.as_ref());

    match service.email_verify(&data.token).await {
        Ok(_) => (StatusCode::OK, Json(json!({ "data":  {} }))).into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, Json(json!({ "data":  err }))).into_response(),
    }
}

async fn send_email_verification(
    State(state): State<Arc<AppState>>,
    JsonInput(body): JsonInput<EmailInputData>,
) -> Response {
    let service = AuthService::default(state.db.users.as_ref());

    match service.send_email_verification(body).await {
        Ok(_) => (StatusCode::OK, Json(json!({ "data":  {} }))).into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, Json(json!({ "data": err}))).into_response(),
    }
}

async fn forgot_password(
    State(state): State<Arc<AppState>>,
    JsonInput(body): JsonInput<EmailInputData>,
) -> Response {
    let service = AuthService::default(state.db.users.as_ref());

    match service.forgot_password(body).await {
        Ok(_) => (StatusCode::OK, Json(json!({ "data":  {} }))).into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, Json(json!({ "data": err}))).into_response(),
    }
}

async fn reset_password(
    State(state): State<Arc<AppState>>,
    auth: AuthData,
    JsonInput(data): JsonInput<PasswordInputData>,
) -> Response {
    let service = AuthService::default(state.db.users.as_ref());
    match service.reset_password(&auth.token, data).await {
        Ok(_) => (StatusCode::OK, Json(json!({ "data":  {} }))).into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, Json(json!({ "data":  err }))).into_response(),
    }
}

async fn revoke_token(State(state): State<Arc<AppState>>, auth: AuthData) -> Response {
    let service = AuthService::default(state.db.users.as_ref());
    match service.revoke_token(&auth.token).await {
        Ok(_) => (StatusCode::OK, Json(json!({ "data":  {} }))).into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, Json(json!({ "data":  err }))).into_response(),
    }
}
