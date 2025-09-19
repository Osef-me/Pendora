use thiserror::Error;

#[derive(Error, Debug)]
pub enum BeatmapWorkerError {
    #[error("Failed to initialize beatmap worker: {0}")]
    InitializationFailed(String),
    
    #[error("Beatmap processing failed: {0}")]
    ProcessingFailed(String),
}
