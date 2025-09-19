use crate::config::Config;
use crate::errors::BeatmapWorkerError;

pub(crate) async fn start(config: &Config) -> Result<(), BeatmapWorkerError> {
    tracing::info!("Beatmap worker started");
    Ok(())
}
