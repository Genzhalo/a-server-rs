use axum::{
    body::Body,
    extract::{State, Path, Query},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json, Router,   
    routing::{post, get},
};
use serde_json::json;
use std::sync::Arc;

use crate::{AppState, app::services::project::{CreateParams, ProjectService, GetProjectByQueryParams}};

use super::extra::{extract::AuthData, json_validate_rejection::JsonInput};

pub fn build_routes() -> Router<Arc<AppState>, Body> {
    Router::new()
    .route("/projects", post(post_project))
    .route("/projects", get(get_projects))
    .route("/projects/:id", get(get_project))
}

async fn post_project(State(state): State<Arc<AppState>>, auth: AuthData, JsonInput(body): JsonInput<CreateParams>) -> Response {
    let service = ProjectService::new(
      state.db.users.as_ref(), 
      state.db.notifications.as_ref(), 
      state.db.projects.as_ref(), 
      &auth.token);
    

    match service.create(body).await {
        Ok(user) => (StatusCode::OK, Json(json!({ "data":  user }))).into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, Json(json!({ "data":  err }))).into_response(),
    }
}

async fn get_project(Path(id): Path<String>, State(state): State<Arc<AppState>>,  auth: AuthData) -> Response {
  let service = ProjectService::new(
    state.db.users.as_ref(), 
    state.db.notifications.as_ref(), 
    state.db.projects.as_ref(), 
    &auth.token);
  

  match service.get_by_id(&id).await {
      Ok(user) => (StatusCode::OK, Json(json!({ "data":  user }))).into_response(),
      Err(err) => (StatusCode::BAD_REQUEST, Json(json!({ "data":  err }))).into_response(),
  }
}


async fn get_projects(Query(params): Query<GetProjectByQueryParams>, State(state): State<Arc<AppState>>, auth: AuthData) -> Response {
  let service: ProjectService<'_> = ProjectService::new(
    state.db.users.as_ref(), 
    state.db.notifications.as_ref(), 
    state.db.projects.as_ref(), 
    &auth.token);
  

  match service.get(&params).await {
      Ok(user) => (StatusCode::OK, Json(json!({ "data":  user }))).into_response(),
      Err(err) => (StatusCode::BAD_REQUEST, Json(json!({ "data":  err }))).into_response(),
  }
}
