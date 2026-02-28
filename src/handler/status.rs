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
    let db = {
        let guard = state.read().await;
        guard.db.clone()
    };

    if let Err(err) = {
        let conn = db.lock().await;
        db::save_state(&conn, &body)
    } {
        return Json(ApiResponse::error(
            format!("Database error: {err}"),
            500,
        ));
    }

    {
        let mut guard = state.write().await;
        guard.status = body.clone();
    }

    Json(ApiResponse::ok_data(
        "Status updated.",
        json!(body),
    ))
}

pub async fn status_patch_handler(
    State(state): State<AppStateStore>,
    Json(body): Json<Status>,
) -> impl IntoResponse {
    let (db, mut next_status) = {
        let guard = state.read().await;
        (guard.db.clone(), guard.status.clone())
    };

    if let Some(h) = body.humidity {
        next_status.humidity = Some(h);
    }
    if let Some(t) = body.temprature {
        next_status.temprature = Some(t);
    }
    if let Some(ms) = body.mainroom_status {
        next_status.mainroom_status = Some(ms);
    }
    if let Some(bs) = body.bathroom_status {
        next_status.bathroom_status = Some(bs);
    }

    if let Err(err) = {
        let conn = db.lock().await;
        db::save_state(&conn, &next_status)
    } {
        return Json(ApiResponse::error(
            format!("Database error: {err}"),
            500,
        ));
    }

    {
        let mut guard = state.write().await;
        guard.status = next_status.clone();
    }

    Json(ApiResponse::ok_data(
        "Status updated.",
        json!(next_status),
    ))
}