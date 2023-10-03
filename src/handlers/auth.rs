use std::sync::Arc;

use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

use crate::{
    core::services::auth::{
        CreateInputData, EmailInputData, LoginInputData, PasswordInputData, UserService,
    },
    db::DB,
};

use super::{extract::AuthData, json_validate_rejection::JsonInput};

pub async fn sign_up(
    State(db): State<Arc<DB>>,
    JsonInput(body): JsonInput<CreateInputData>,
) -> Response {
    let service = UserService::default(db.user.as_ref());

    match service.create(body).await {
        Ok(id) => (StatusCode::OK, Json(json!({"data": { "id": id }}))).into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, Json(json!({ "data": err }))).into_response(),
    }
}

pub async fn sign_in(
    State(db): State<Arc<DB>>,
    JsonInput(body): JsonInput<LoginInputData>,
) -> Response {
    let service = UserService::default(db.user.as_ref());

    match service.login(body).await {
        Ok(token) => (StatusCode::OK, Json(json!({"data": token}))).into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, Json(json!({ "data":  err }))).into_response(),
    }
}

pub async fn email_verify(State(db): State<Arc<DB>>, data: AuthData) -> Response {
    let service = UserService::default(db.user.as_ref());

    match service.email_verify(&data.token).await {
        Ok(_) => (StatusCode::OK, Json(json!({ "data":  {} }))).into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, Json(json!({ "data":  err }))).into_response(),
    }
}

pub async fn send_email_verification(
    State(db): State<Arc<DB>>,
    JsonInput(body): JsonInput<EmailInputData>,
) -> Response {
    let service = UserService::default(db.user.as_ref());

    match service.send_email_verification(body).await {
        Ok(_) => (StatusCode::OK, Json(json!({ "data":  {} }))).into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, Json(json!({ "data": err}))).into_response(),
    }
}

pub async fn forgot_password(
    State(db): State<Arc<DB>>,
    JsonInput(body): JsonInput<EmailInputData>,
) -> Response {
    let service = UserService::default(db.user.as_ref());

    match service.forgot_password(body).await {
        Ok(_) => (StatusCode::OK, Json(json!({ "data":  {} }))).into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, Json(json!({ "data": err}))).into_response(),
    }
}

pub async fn reset_password(
    State(db): State<Arc<DB>>,
    auth: AuthData,
    JsonInput(data): JsonInput<PasswordInputData>,
) -> Response {
    let service = UserService::default(db.user.as_ref());
    match service.reset_password(&auth.token, data).await {
        Ok(_) => (StatusCode::OK, Json(json!({ "data":  {} }))).into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, Json(json!({ "data":  err }))).into_response(),
    }
}

pub async fn revoke_token(State(db): State<Arc<DB>>, auth: AuthData) -> Response {
    let service = UserService::default(db.user.as_ref());
    match service.revoke_token(&auth.token).await {
        Ok(_) => (StatusCode::OK, Json(json!({ "data":  {} }))).into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, Json(json!({ "data":  err }))).into_response(),
    }
}
