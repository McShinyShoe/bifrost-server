use axum::{Json, extract::State, response::IntoResponse};
use serde_json::json;

use crate::{api_response::{ApiResponse, JsonApiResponse}, app_state::AppStateStore, db, status::Status};

pub async fn status_get_handler(State(state): State<AppStateStore>) -> JsonApiResponse {
    Json(ApiResponse::ok_data(
        "Status retrieved.",
        json!(state.read().await.status),
    ))
}
pub async fn status_put_handler(
    State(state): State<AppStateStore>,
    Json(body): Json<Status>,
) -> impl IntoResponse {
    let mut guard = state.write().await;

    guard.status = body;
    let db = guard.db.clone();

    match db::save_state(&*db.lock().await, &guard.status) {
        Ok(_) => {}
        Err(err) => {
            ApiResponse::error(format!("Database Error: {}", err.to_string()), 500);
        }
    }

    Json(ApiResponse::ok_data(
        "Status updated.",
        json!(guard.status.clone()),
    ))
}
pub async fn status_patch_handler(
    State(state): State<AppStateStore>,
    Json(body): Json<Status>,
) -> impl IntoResponse {
    let mut guard = state.write().await;
    let db = guard.db.clone();

    if let Some(h) = body.humidity {
        guard.status.humidity = Some(h);
    }
    
    if let Some(t) = body.temprature {
        guard.status.temprature = Some(t);
    }
    
    if let Some(ms) = body.mainroom_status {
        guard.status.mainroom_status = Some(ms);
    }
    
    if let Some(bs) = body.bathroom_status {
        guard.status.bathroom_status = Some(bs);
    }

    match db::save_state(&*db.lock().await, &guard.status) {
        Ok(_) => {}
        Err(err) => {
            ApiResponse::error(format!("Database Error: {}", err.to_string()), 500);
        }
    }

    Json(ApiResponse::ok_data(
        "Status updated.",
        json!(guard.status.clone()),
    ))
}
