use axum::http::StatusCode;
use axum::routing::get;
use axum::{Json, Router};

use crate::home_state::HOME_STATE;

type ApiError = (StatusCode, String);

fn internal(e: std::io::Error) -> ApiError {
    (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
}

pub fn router<S: Clone + Send + Sync + 'static>() -> Router<S> {
    Router::new()
        .route("/light/mainroom", get(get_mainroom).put(put_mainroom))
        .route("/light/bathroom", get(get_bathroom).put(put_bathroom))
        .route("/sensor/temperature", get(get_temperature).put(put_temperature))
        .route("/sensor/humidity", get(get_humidity).put(put_humidity))
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
