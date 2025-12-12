#![allow(unused)]
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Status {
    pub humidity: Option<f64>,
    pub temprature: Option<f64>,
}
