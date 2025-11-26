use axum::{Json, extract::State};
use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::Deserialize;
use serde_json::json;

use crate::{app_state::AppStateStore, claims::Claims, api_response::ApiResponse};

#[derive(Deserialize)]
pub struct LoginInput {
    username: String,
    password: String,
}

pub async fn login_handler(
    State(state): State<AppStateStore>,
    Json(body): Json<LoginInput>,
) -> Json<ApiResponse> {
    if body.username != "admin" || body.password != "password" {
        return Json(ApiResponse::error("wrong username/password", 500));
    }

    let expiration = (Utc::now() + Duration::hours(24)).timestamp() as usize;

    let claims = Claims {
        sub: body.username,
        exp: expiration,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(state.read().await.secret.as_bytes()),
    )
    .unwrap();

    Json(ApiResponse::ok_data(
        "Login successfull.",
        json!({ "token": token }),
    ))
}
