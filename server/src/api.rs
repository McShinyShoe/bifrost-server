use std::time::{SystemTime, UNIX_EPOCH};

use axum::extract::Path;
use axum::http::StatusCode;
use axum::routing::get;
use axum::{Json, Router};
use serde::Deserialize;

use crate::home_state::{AcCommand, HOME_STATE};

type ApiError = (StatusCode, String);

fn internal(e: std::io::Error) -> ApiError {
    (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
}

fn no_command(id: u32) -> ApiError {
    (StatusCode::NOT_FOUND, format!("no command with id {id}"))
}

pub fn router<S: Clone + Send + Sync + 'static>() -> Router<S> {
    Router::new()
        .route("/light/mainroom", get(get_mainroom).put(put_mainroom))
        .route("/light/bathroom", get(get_bathroom).put(put_bathroom))
        .route("/sensor/temperature", get(get_temperature).put(put_temperature))
        .route("/sensor/humidity", get(get_humidity).put(put_humidity))
        .route(
            "/ac/command",
            get(get_commands).post(post_command).put(put_commands),
        )
        .route(
            "/ac/command/{id}",
            get(get_command).put(put_command).delete(delete_command),
        )
}

async fn get_mainroom() -> Result<Json<bool>, ApiError> {
    let state = HOME_STATE.read().await.map_err(internal)?;
    Ok(Json(state.light.mainroom))
}

async fn put_mainroom(Json(value): Json<bool>) -> Result<Json<bool>, ApiError> {
    let state = HOME_STATE
        .update(|s| s.light.mainroom = value)
        .await
        .map_err(internal)?;
    Ok(Json(state.light.mainroom))
}

async fn get_bathroom() -> Result<Json<bool>, ApiError> {
    let state = HOME_STATE.read().await.map_err(internal)?;
    Ok(Json(state.light.bathroom))
}

async fn put_bathroom(Json(value): Json<bool>) -> Result<Json<bool>, ApiError> {
    let state = HOME_STATE
        .update(|s| s.light.bathroom = value)
        .await
        .map_err(internal)?;
    Ok(Json(state.light.bathroom))
}

async fn get_temperature() -> Result<Json<f32>, ApiError> {
    let state = HOME_STATE.read().await.map_err(internal)?;
    Ok(Json(state.sensor.temperature))
}

async fn put_temperature(Json(value): Json<f32>) -> Result<Json<f32>, ApiError> {
    let state = HOME_STATE
        .update(|s| s.sensor.temperature = value)
        .await
        .map_err(internal)?;
    Ok(Json(state.sensor.temperature))
}

async fn get_humidity() -> Result<Json<f32>, ApiError> {
    let state = HOME_STATE.read().await.map_err(internal)?;
    Ok(Json(state.sensor.humidity))
}

async fn put_humidity(Json(value): Json<f32>) -> Result<Json<f32>, ApiError> {
    let state = HOME_STATE
        .update(|s| s.sensor.humidity = value)
        .await
        .map_err(internal)?;
    Ok(Json(state.sensor.humidity))
}

async fn get_commands() -> Result<Json<Vec<AcCommand>>, ApiError> {
    let state = HOME_STATE.read().await.map_err(internal)?;
    Ok(Json(state.ac.queue))
}

// either "command" or {"command": "...", "timestamp": 123}, timestamp defaults to now
#[derive(Deserialize)]
#[serde(
    untagged,
    expecting = "a command string, or an object with a command and an optional timestamp"
)]
enum NewCommand {
    Bare(String),
    Full { command: String, timestamp: Option<u64> },
}

impl NewCommand {
    fn into_parts(self, now: u64) -> (String, u64) {
        match self {
            NewCommand::Bare(command) => (command, now),
            NewCommand::Full { command, timestamp } => (command, timestamp.unwrap_or(now)),
        }
    }
}

fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or_default()
}

async fn post_command(
    Json(command): Json<NewCommand>,
) -> Result<(StatusCode, Json<AcCommand>), ApiError> {
    let (command, timestamp) = command.into_parts(now());
    let entry = HOME_STATE
        .update_with(|s| s.ac.push(command, timestamp))
        .await
        .map_err(internal)?;
    Ok((StatusCode::CREATED, Json(entry)))
}

async fn put_commands(
    Json(commands): Json<Vec<NewCommand>>,
) -> Result<Json<Vec<AcCommand>>, ApiError> {
    let now = now();
    let queue = HOME_STATE
        .update_with(|s| s.ac.replace(commands.into_iter().map(|c| c.into_parts(now))))
        .await
        .map_err(internal)?;
    Ok(Json(queue))
}

async fn get_command(Path(id): Path<u32>) -> Result<Json<AcCommand>, ApiError> {
    let state = HOME_STATE.read().await.map_err(internal)?;
    state.ac.get(id).cloned().map(Json).ok_or_else(|| no_command(id))
}

async fn put_command(
    Path(id): Path<u32>,
    Json(command): Json<String>,
) -> Result<Json<AcCommand>, ApiError> {
    HOME_STATE
        .update_with(|s| s.ac.set(id, command))
        .await
        .map_err(internal)?
        .map(Json)
        .ok_or_else(|| no_command(id))
}

async fn delete_command(Path(id): Path<u32>) -> Result<Json<AcCommand>, ApiError> {
    HOME_STATE
        .update_with(|s| s.ac.remove(id))
        .await
        .map_err(internal)?
        .map(Json)
        .ok_or_else(|| no_command(id))
}
