use std::io;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;

use serde::{Deserialize, Deserializer, Serialize};
use tokio::sync::Mutex;

const HOME_STATE_PATH: &str = "data/home_state.json";

pub static HOME_STATE: LazyLock<HomeStateStore> =
    LazyLock::new(|| HomeStateStore::new(HOME_STATE_PATH));

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HomeState {
    pub light: Light,
    pub sensor: Sensor,
    // so that state files from before the queue exist still loads
    #[serde(default)]
    pub ac: Ac,
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

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Ac {
    // sorted again on load in case the file got edited by hand
    #[serde(deserialize_with = "deserialize_sorted")]
    pub queue: Vec<AcCommand>,
    pub last_id: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcCommand {
    pub id: u32,
    pub command: String,
    // unix seconds, entries from before timestamps existed get 0
    #[serde(default)]
    pub timestamp: u64,
}

fn sort_queue(queue: &mut [AcCommand]) {
    // ties go by id, so by the order they got added
    queue.sort_by_key(|c| (c.timestamp, c.id));
}

fn deserialize_sorted<'de, D: Deserializer<'de>>(d: D) -> Result<Vec<AcCommand>, D::Error> {
    let mut queue = Vec::<AcCommand>::deserialize(d)?;
    sort_queue(&mut queue);
    Ok(queue)
}

impl Ac {
    pub fn push(&mut self, command: String, timestamp: u64) -> AcCommand {
        self.last_id += 1;
        let entry = AcCommand { id: self.last_id, command, timestamp };
        self.queue.push(entry.clone());
        sort_queue(&mut self.queue);
        entry
    }

    pub fn get(&self, id: u32) -> Option<&AcCommand> {
        self.queue.iter().find(|c| c.id == id)
    }

    pub fn set(&mut self, id: u32, command: String) -> Option<AcCommand> {
        let entry = self.queue.iter_mut().find(|c| c.id == id)?;
        entry.command = command;
        Some(entry.clone())
    }

    pub fn remove(&mut self, id: u32) -> Option<AcCommand> {
        let i = self.queue.iter().position(|c| c.id == id)?;
        Some(self.queue.remove(i))
    }

    pub fn replace(&mut self, commands: impl IntoIterator<Item = (String, u64)>) -> Vec<AcCommand> {
        self.queue.clear();
        for (command, timestamp) in commands {
            self.push(command, timestamp);
        }
        self.queue.clone()
    }

    pub fn clear(&mut self) -> Vec<AcCommand> {
        std::mem::take(&mut self.queue)
    }
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
        self.update_with(|s| {
            f(s);
            s.clone()
        })
        .await
    }

    /// Same as `update`, but returns whatever `f` returns instead of the whole state.
    pub async fn update_with<T>(&self, f: impl FnOnce(&mut HomeState) -> T) -> io::Result<T> {
        let path = self.path.lock().await;
        let mut state = read_file(&path).await?;
        let out = f(&mut state);
        write_file(&path, &state).await?;
        Ok(out)
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
