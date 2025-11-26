mod app_state;
mod claims;
mod handler;
mod middleware;
mod response;

use crate::{
    app_state::{AppState, AppStateStore},
    handler::login::login_handler,
    response::Response,
};
use axum::{
    Json, Router,
    extract::State,
    middleware::from_fn_with_state,
    routing::{get, post},
};
use std::{collections::HashMap, sync::Arc};
use tokio::{net::TcpListener, sync::RwLock};

use axum::response::IntoResponse;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    tracing::info!("Starting Server ...");
    let state = Arc::new(RwLock::new(AppState {
        sessions: HashMap::new(),
        secret: "SECRET".to_owned(),
    }));

    let app = Router::new()
        .route("/login", post(login_handler))
        .route(
            "/hello",
            get(hello).layer(from_fn_with_state(
                state.clone(),
                middleware::auth::auth_middleware,
            )),
        )
        .with_state(state);

    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
async fn hello(State(_): State<AppStateStore>) -> impl IntoResponse {
    Json(Response::ok("Hello! You passed authentication."))
}
