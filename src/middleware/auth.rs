use axum::{
    Json,
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};

use crate::{api_response::ApiResponse, app_state::AppStateStore, claims::Claims};
use jsonwebtoken::{DecodingKey, Validation, decode};

pub async fn auth_middleware(
    State(state): State<AppStateStore>,
    req: Request<Body>,
    next: Next,
) -> Result<Response, (StatusCode, Json<ApiResponse>)> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok());

    let Some(auth) = auth_header else {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(ApiResponse::error(
                StatusCode::UNAUTHORIZED.canonical_reason().unwrap_or("Error"),
                StatusCode::UNAUTHORIZED.as_u16(),
            )),
        ));
    };

    let token = auth.strip_prefix("Bearer ").unwrap_or("");

    let decoded = decode::<Claims>(
        token,
        &DecodingKey::from_secret(state.read().await.secret.as_bytes()),
        &Validation::default(),
    );

    let _ = decoded.map_err(|_| {
        (
            StatusCode::UNAUTHORIZED,
            Json(ApiResponse::error(
                StatusCode::UNAUTHORIZED.canonical_reason().unwrap_or("Error"),
                StatusCode::UNAUTHORIZED.as_u16(),
            )),
        )
    })?;

    Ok(next.run(req).await)
}
