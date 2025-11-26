#![allow(unused)]
use crate::{claims::Claims, status::Status};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub struct AppState {
    pub sessions: HashMap<String, Claims>,
    pub secret: String,
    pub status: Status,
}
pub type AppStateStore = Arc<RwLock<AppState>>;
