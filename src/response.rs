#![allow(unused)]
use serde::Serialize;

#[derive(Serialize)]
pub struct Response {
    pub code: u16,
    pub message: String,
    pub data: Option<serde_json::Value>,
}

impl Response {
    pub fn ok(message: impl Into<String>) -> Self {
        Self {
            code: 200,
            message: message.into(),
            data: None,
        }
    }
    pub fn ok_data<D: Serialize>(message: impl Into<String>, data: D) -> Self {
        Self {
            code: 200,
            message: message.into(),
            data: Some(serde_json::to_value(data).unwrap()),
        }
    }

    pub fn error(message: impl Into<String>, code: u16) -> Self {
        Self {
            code,
            message: message.into(),
            data: None,
        }
    }
    pub fn error_data<D: Serialize>(message: impl Into<String>, code: u16, data: D) -> Self {
        Self {
            code,
            message: message.into(),
            data: Some(serde_json::to_value(data).unwrap()),
        }
    }
}
