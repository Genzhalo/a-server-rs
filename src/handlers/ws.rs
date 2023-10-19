use std::sync::Arc;

use crate::{
    app::{entities::user::User, services::user::UserService},
    AppState, Event,
};
use axum::{
    body::Body,
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use futures::{sink::SinkExt, stream::StreamExt};
use serde_json::json;

use super::extra::extract::AuthData;

pub fn build_routes() -> Router<Arc<AppState>, Body> {
    Router::new().route("/subscribe", get(live))
}

async fn live(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
    auth: AuthData,
) -> impl IntoResponse {
    let service = UserService::new(state.db.users.as_ref(), &auth.token);

    let user = match service.get_current_user().await {
        Ok(user) => user,
        Err(err) => return (StatusCode::FORBIDDEN, Json(json!({ "data":  err }))).into_response(),
    };

    ws.on_upgrade(|socket| websocket(socket, state, user))
}

async fn websocket(stream: WebSocket, state: Arc<AppState>, _user: User) {
    let (mut sender, mut receiver) = stream.split();

    let mut rx = state.tx.subscribe();

    let mut recv_task = tokio::spawn(async move {
        while let Some(msq) = receiver.next().await {
            if msq.is_err() {
                break;
            }
        }
    });

    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            let notification = match msg {
                Event::CreateNotification(n) => n,
            };

            if sender
                .send(Message::Text(json!(notification).to_string()))
                .await
                .is_err()
            {
                break;
            }
        }
    });

    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    }
}
