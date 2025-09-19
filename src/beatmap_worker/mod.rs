
use crate::errors::BeatmapWorkerError;

pub(crate) async fn start() -> Result<(), BeatmapWorkerError> {
    println!("Beatmap worker started");
    Ok(())
}