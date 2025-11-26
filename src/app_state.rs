#![allow(unused)]
use crate::claims::Claims;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub struct AppState {
    pub sessions: HashMap<String, Claims>,
    pub secret: String,
}
pub type AppStateStore = Arc<RwLock<AppState>>;
