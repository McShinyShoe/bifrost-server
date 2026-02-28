#![allow(unused)]
use crate::{appconfig::AppConfig, claims::Claims, status::Status};
use std::{collections::HashMap, sync::Arc};
use rusqlite::Connection;
use tokio::sync::{Mutex, RwLock};

#[derive(Debug, Clone)]
pub struct AppState {
    pub sessions: HashMap<String, Claims>,
    pub secret: String,
    pub status: Status,
    pub config: AppConfig,

    pub db: Arc<Mutex<Connection>>,
}
pub type AppStateStore = Arc<RwLock<AppState>>;
