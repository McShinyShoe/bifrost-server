use axum::{Json, extract::State, response::IntoResponse};

use crate::{app_state::AppStateStore, response::Response};

pub async fn hello_handler(State(_): State<AppStateStore>) -> impl IntoResponse {
    Json(Response::ok("Hello World."))
}
