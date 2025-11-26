#![allow(unused)]
use config::{Config, Environment, File};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub jwt_secret: String,
    pub port: u16,
    pub username: String,
    pub password: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok(); // load .env if present

        Config::builder()
            .add_source(File::with_name("Config").required(false)) // optional
            .add_source(Environment::default()) // loads from ENV VARs
            .build()
            .unwrap()
            .try_deserialize()
            .unwrap()
    }
}
