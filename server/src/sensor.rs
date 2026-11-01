use std::io;
use std::sync::LazyLock;
use std::time::Duration;

use leptos::logging::warn;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::Notify;
use tokio::time::timeout;

const DEFAULT_SENSOR_ADDR: &str = "10.10.0.5:80";
const TIMEOUT: Duration = Duration::from_secs(5);

pub static SENSOR_ADDR: LazyLock<String> = LazyLock::new(|| {
    std::env::var("BIFROST_SENSOR_ADDR").unwrap_or_else(|_| DEFAULT_SENSOR_ADDR.to_owned())
});

static POLL_WANTED: Notify = Notify::const_new();

pub async fn request_poll() -> io::Result<()> {
    timeout(TIMEOUT, send_poll(&SENSOR_ADDR))
        .await
        .unwrap_or_else(|_| Err(io::ErrorKind::TimedOut.into()))
}

async fn send_poll(addr: &str) -> io::Result<()> {
    let mut stream = TcpStream::connect(addr).await?;
    let head = format!("POST /poll HTTP/1.0\r\nHost: {addr}\r\nContent-Length: 0\r\n\r\n");
    stream.write_all(head.as_bytes()).await?;

    let mut response = Vec::new();
    stream.take(256).read_to_end(&mut response).await?;

    let status = std::str::from_utf8(&response)
        .ok()
        .and_then(|r| r.strip_prefix("HTTP/1."))
        .and_then(|r| r.get(2..5))
        .and_then(|s| s.parse::<u16>().ok());
    match status {
        Some(200..=299) => Ok(()),
        Some(status) => Err(io::Error::other(format!("it answered {status}"))),
        None => Err(io::Error::new(io::ErrorKind::InvalidData, "it didnt answer with http")),
    }
}

pub fn request_poll_later() {
    POLL_WANTED.notify_one();
}

pub async fn run_background_polls() {
    loop {
        POLL_WANTED.notified().await;
        if let Err(e) = request_poll().await {
            warn!("couldnt tell the sensor at {} to poll: {e}", *SENSOR_ADDR);
        }
    }
}
