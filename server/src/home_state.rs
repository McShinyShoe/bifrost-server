use std::io;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;

use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

const HOME_STATE_PATH: &str = "data/home_state.json";

pub static HOME_STATE: LazyLock<HomeStateStore> =
    LazyLock::new(|| HomeStateStore::new(HOME_STATE_PATH));

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HomeState {
    pub light: Light,
    pub sensor: Sensor,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Light {
    pub mainroom: bool,
    pub bathroom: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Sensor {
    pub temperature: f32,
    pub humidity: f32,
}

pub struct HomeStateStore {
    path: Mutex<PathBuf>,
}

impl HomeStateStore {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: Mutex::new(path.into()) }
    }

    pub async fn read(&self) -> io::Result<HomeState> {
        let path = self.path.lock().await;
        read_file(&path).await
    }

    pub async fn write(&self, state: &HomeState) -> io::Result<()> {
        let path = self.path.lock().await;
        write_file(&path, state).await
    }

    pub async fn update(&self, f: impl FnOnce(&mut HomeState)) -> io::Result<HomeState> {
        let path = self.path.lock().await;
        let mut state = read_file(&path).await?;
        f(&mut state);
        write_file(&path, &state).await?;
        Ok(state)
    }
}

async fn read_file(path: &Path) -> io::Result<HomeState> {
    match tokio::fs::read(path).await {
        Ok(bytes) => Ok(serde_json::from_slice(&bytes)?),
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(HomeState::default()),
        Err(e) => Err(e),
    }
}

async fn write_file(path: &Path, state: &HomeState) -> io::Result<()> {
    if let Some(dir) = path.parent() {
        tokio::fs::create_dir_all(dir).await?;
    }

    let tmp = path.with_extension("json.tmp");
    tokio::fs::write(&tmp, serde_json::to_vec_pretty(state)?).await?;
    tokio::fs::rename(&tmp, path).await
}
