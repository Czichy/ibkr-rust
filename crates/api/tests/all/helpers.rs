use std::net::SocketAddr;

use ibkr_rust_api::{client, client::Client, Result};

/// TWS Paper-Trading Gateway address (default: localhost:4002).
/// Override via `TWS_HOST` and `TWS_PORT` environment variables.
pub fn tws_addr() -> SocketAddr {
    let host = std::env::var("TWS_HOST").unwrap_or_else(|_| "127.0.0.1".into());
    let port: u16 = std::env::var("TWS_PORT")
        .unwrap_or_else(|_| "4002".into())
        .parse()
        .expect("TWS_PORT must be a valid u16");
    SocketAddr::new(host.parse().expect("TWS_HOST must be a valid IP"), port)
}

/// Connect to TWS with a unique client_id.
pub async fn connect_test_client(client_id: i32) -> Result<Client> {
    client::connect(tws_addr(), client_id).await
}

/// Wait for a value from a flume channel with a timeout.
/// Returns `None` if the timeout expires or the channel is disconnected.
pub async fn recv_timeout<T>(rx: &flume::Receiver<T>, timeout_ms: u64) -> Option<T> {
    tokio::time::timeout(
        std::time::Duration::from_millis(timeout_ms),
        rx.recv_async(),
    )
    .await
    .ok()
    .and_then(|r| r.ok())
}
