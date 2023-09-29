use dotenv::dotenv;
use qbit_rs::{model::ConnectionStatus, model::Credential, Qbit};
use std::env;
use tracing::{error, info};
use url::Url;

struct QbitConfig {
    endpoint: Url,
    username: String,
    password: String,
    check_interval: u64,
    retry_interval: u64,
    shutdown_wait: u64,
}

enum ConnectionCheckResult {
    Connected,
    Disconnected,
    Error,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let config = QbitConfig {
        endpoint: Url::parse(&env::var("QBIT_ENDPOINT").expect("QBIT_ENDPOINT must be set"))
            .expect("QBIT_ENDPOINT must be a valid url"),
        username: env::var("QBIT_USERNAME").expect("QBIT_USERNAME must be set"),
        password: env::var("QBIT_PASSWORD").expect("QBIT_PASSWORD must be set"),
        check_interval: env::var("QBIT_CHECK_INTERVAL")
            .unwrap_or("60".to_string())
            .parse::<u64>()
            .expect("QBIT_CHECK_INTERVAL must be a number"),
        retry_interval: env::var("QBIT_RETRY_INTERVAL")
            .unwrap_or("5".to_string())
            .parse::<u64>()
            .expect("QBIT_RETRY_INTERVAL must be a number"),
        shutdown_wait: env::var("QBIT_SHUTDOWN_WAIT")
            .unwrap_or("30".to_string())
            .parse::<u64>()
            .expect("QBIT_SHUTDOWN_WAIT must be a number"),
    };

    info!("Starting qBittorrent connection monitor");
    info!("Connecting to qBittorrent at {}", config.endpoint);
    let credential = Credential::new(config.username, config.password);
    let qbit = Qbit::new(config.endpoint, credential);

    let login_result = qbit.login(true).await;

    if login_result.is_err() {
        error!(
            "Failed to login to qBittorrent: {:?}. Check connection and credentials.",
            login_result.err()
        );
        return;
    }

    info!("Successfully logged in to qBittorrent. Starting connection monitor.");

    loop {
        let wait_time = match check_connection_status(&qbit).await {
            ConnectionCheckResult::Connected => config.check_interval,
            ConnectionCheckResult::Disconnected => {
                shutdown_qbittorrent(&qbit).await;
                config.shutdown_wait
            }
            ConnectionCheckResult::Error => config.retry_interval,
        };

        tokio::time::sleep(tokio::time::Duration::from_secs(wait_time)).await;
    }
}

async fn check_connection_status(qbit: &Qbit) -> ConnectionCheckResult {
    info!("Checking qBittorrent status");
    let transfer_info = qbit.get_transfer_info().await;

    if transfer_info.is_err() {
        error!(
            "Failed to get status from qBittorrent: {:?}",
            transfer_info.err(),
        );
        return ConnectionCheckResult::Error;
    }

    match transfer_info.unwrap().connection_status {
        ConnectionStatus::Connected => {
            info!("qBittorrent is connected to the internet");
            ConnectionCheckResult::Connected
        }
        ConnectionStatus::Disconnected | ConnectionStatus::Firewalled => {
            info!("qBittorrent is not connected to the internet");
            ConnectionCheckResult::Disconnected
        }
        ConnectionStatus::Unknown => {
            error!("qBittorrent returned unknown connection status");
            ConnectionCheckResult::Error
        }
    }
}

async fn shutdown_qbittorrent(qbit: &Qbit) {
    info!("Shutting down qBittorrent");
    let shutdown_result = qbit.shutdown().await;

    if shutdown_result.is_err() {
        error!(
            "Failed to shutdown qBittorrent: {:?}",
            shutdown_result.err()
        );
    }
}
