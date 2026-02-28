mod app_state;
mod claims;
mod handler;
mod middleware;
mod api_response;
mod status;
mod appconfig;
mod db;

use axum::{
    Router,
    middleware::from_fn_with_state,
    routing::{get, post, put, patch},
};
use std::{collections::HashMap, fs, sync::Arc};
use tokio::{net::TcpListener, sync::RwLock};

use crate::{
    app_state::AppState, appconfig::AppConfig, handler::{hello::hello_handler, login::login_handler, status::{status_get_handler, status_patch_handler, status_put_handler}}
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    tracing::info!("Starting Server ...");
    let config = AppConfig::from_env();
    let addr = format!("0.0.0.0:{}", config.port);

    fs::create_dir_all("/var/lib/bifrost")?;
    let db = db::make_connection("/var/lib/bifrost/database.db")?;
    let status = db::get_status(&db)?;

    let state = Arc::new(RwLock::new(AppState {
        sessions: HashMap::new(),
        secret: "SECRET".to_owned(),
        status: status,
        config: config,
        db: Arc::new(tokio::sync::Mutex::new(db))
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
        .route("/status", put(status_put_handler).layer(from_fn_with_state(
                state.clone(),
                middleware::auth::auth_middleware,
            ))
        )
        .route("/status", patch(status_patch_handler).layer(from_fn_with_state(
                state.clone(),
                middleware::auth::auth_middleware,
            ))
        )
        .with_state(state);

    tracing::info!("Listening on {}", &addr);
    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}