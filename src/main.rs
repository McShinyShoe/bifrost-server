mod app_state;
mod claims;
mod handler;
mod middleware;
mod api_response;
mod status;

use axum::{
    Router,
    middleware::from_fn_with_state,
    routing::{get, post},
};
use std::{collections::HashMap, sync::Arc};
use tokio::{net::TcpListener, sync::RwLock};

use crate::{
    app_state::AppState,
    handler::{hello::hello_handler, login::login_handler, status::{status_get_handler, status_update_handler}}, status::Status,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    tracing::info!("Starting Server ...");
    let state = Arc::new(RwLock::new(AppState {
        sessions: HashMap::new(),
        secret: "SECRET".to_owned(),
        status: Status {
            humidity: 0.0,
            temprature: 0.0,
        }
    }));

    let app = Router::new()
        .route("/login", post(login_handler))
        .route(
            "/hello",
            get(hello_handler).layer(from_fn_with_state(
                state.clone(),
                middleware::auth::auth_middleware,
            )),
        )
        .route("/status", get(status_get_handler).layer(from_fn_with_state(
                state.clone(),
                middleware::auth::auth_middleware,
            ))
        )
        .route("/status", post(status_update_handler).layer(from_fn_with_state(
                state.clone(),
                middleware::auth::auth_middleware,
            ))
        )
        .with_state(state);

    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}