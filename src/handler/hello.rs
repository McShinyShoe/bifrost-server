use axum::{Json, extract::State, response::IntoResponse};

use crate::{app_state::AppStateStore, api_response::ApiResponse};

pub async fn hello_handler(State(_): State<AppStateStore>) -> impl IntoResponse {
    Json(ApiResponse::ok("Hello World."))
}
