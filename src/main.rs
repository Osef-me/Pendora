mod api;
mod config;
mod core;
mod errors;
mod utils;

use api::osu::OsuApiService;
use config::Config;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{fmt, layer::SubscriberExt, EnvFilter};

fn init_logging() -> tracing_appender::non_blocking::WorkerGuard {
    // Create logs directory if it doesn't exist
    std::fs::create_dir_all("logs").unwrap_or_else(|_| {});

    // Create file appender that rotates daily
    let file_appender = RollingFileAppender::new(Rotation::DAILY, "logs", "pendora.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    // Initialize tracing with both console and file output
    let subscriber = tracing_subscriber::registry()
        .with(fmt::layer().with_writer(std::io::stdout).with_ansi(true))
        .with(fmt::layer().with_writer(non_blocking).with_ansi(false))
        .with(EnvFilter::new("info"));

    tracing::subscriber::set_global_default(subscriber).expect("Setting default subscriber failed");

    guard
}

#[tokio::main]
async fn main() {
    let _guard = init_logging();

    // Load configuration
    let config = match Config::load().await {
        Ok(config) => config,
        Err(e) => {
            tracing::error!("Error while loading config: {}", e);
            std::process::exit(1);
        }
    };

    let osu_api_service = OsuApiService::new(
        config.osu_client_id.clone(),
        config.osu_client_secret.clone(),
    )
    .await
    .unwrap();
    tracing::info!("Application started successfully");

    let beatmap_worker = core::worker::BeatmapWorker {
        config,
        osu_api_service,
    };
    tokio::select! {
        result = beatmap_worker.start("6dd2938dfb35fc68465974a431096d89".to_string()) => {
            tracing::info!("BeatmapWorker finished: {:?}", result);
        }
    }
}
