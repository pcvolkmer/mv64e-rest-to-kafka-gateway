use rdkafka::ClientConfig;
use rdkafka::producer::FutureProducer;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, LazyLock};

#[cfg(not(test))]
use clap::Parser;

use crate::auth::is_valid_brypt_hash;
use crate::cli::Cli;
use crate::sender::DefaultMtbFileSender;
use crate::server::start_server;

mod auth;
mod cli;
mod sender;
mod server;

#[derive(Serialize, Deserialize)]
struct RecordKey {
    #[serde(rename = "pid")]
    patient_id: String,
}

#[cfg(not(test))]
static CONFIG: LazyLock<Cli> = LazyLock::new(Cli::parse);

#[tokio::main]
async fn main() -> Result<(), ()> {
    #[cfg(debug_assertions)]
    {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::TRACE)
            .init();
    }

    #[cfg(not(debug_assertions))]
    {
        use std::str::FromStr;

        let trace_level = tracing::Level::from_str(
            &std::env::var("LOG_LEVEL").unwrap_or_else(|_| "INFO".to_owned()),
        )
        .unwrap_or(tracing::Level::INFO);

        tracing_subscriber::fmt().with_max_level(trace_level).init();
    }

    if !is_valid_brypt_hash(&CONFIG.token) {
        log::error!("Error starting application: given token is not a valid BCrypt token");
        return Err(());
    }

    if let Err(err_msg) = start_service().await {
        log::error!("Error starting service: {err_msg}");
    }

    Ok(())
}

async fn start_service() -> Result<(), String> {
    let mut client_config = ClientConfig::new();

    client_config
        .set("bootstrap.servers", &CONFIG.bootstrap_server)
        .set("message.timeout.ms", "5000");

    let producer = if CONFIG.ssl_cert_file.is_some() || CONFIG.ssl_key_file.is_some() {
        // Use SSL
        client_config
            .set("security.protocol", "ssl")
            .set(
                "ssl.ca.location",
                CONFIG.ssl_ca_file.clone().unwrap_or_default(),
            )
            .set(
                "ssl.certificate.location",
                CONFIG.ssl_cert_file.clone().unwrap_or_default(),
            )
            .set(
                "ssl.key.location",
                CONFIG.ssl_key_file.clone().unwrap_or_default(),
            );
        if let Some(ssl_key_password) = &CONFIG.ssl_key_password {
            client_config.set("ssl.key.password", ssl_key_password);
        }
        client_config
            .create::<FutureProducer>()
            .map_err(|err| err.to_string())?
    } else {
        // Plain
        client_config
            .create::<FutureProducer>()
            .map_err(|err| err.to_string())?
    };

    start_server(Arc::new(DefaultMtbFileSender::new(&CONFIG.topic, producer))).await
}

#[allow(clippy::expect_used)]
async fn shutdown_signal() {
    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    terminate.await;
}

// Test Configuration
#[cfg(test)]
static CONFIG: LazyLock<Cli> = LazyLock::new(|| Cli {
    bootstrap_server: "localhost:9094".to_string(),
    topic: "test-topic".to_string(),
    // Basic dG9rZW46dmVyeS1zZWNyZXQ=
    token: "$2y$05$LIIFF4Rbi3iRVA4UIqxzPeTJ0NOn/cV2hDnSKFftAMzbEZRa42xSG".to_string(),
    listen: "0.0.0.0:3000".to_string(),
    ssl_ca_file: None,
    ssl_cert_file: None,
    ssl_key_file: None,
    ssl_key_password: None,
    send_on_invalid: true,
});
