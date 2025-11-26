use axum::{Json, extract::State, response::IntoResponse};
use serde_json::json;

use crate::{app_state::AppStateStore, response::Response, status::Status};

pub async fn status_get_handler(State(state): State<AppStateStore>) -> impl IntoResponse {
    Json(Response::ok_data(
        "Status retrieved.",
        json!(state.read().await.status),
    ))
}
pub async fn status_update_handler(
    State(state): State<AppStateStore>,
    Json(body): Json<Status>,
) -> impl IntoResponse {
    let mut guard = state.write().await;

    guard.status.humidity = body.humidity;
    guard.status.temprature = body.temprature;

    Json(Response::ok_data(
        "Status updated.",
        json!(guard.status.clone()),
    ))
}
