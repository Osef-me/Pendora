use thiserror::Error;

#[derive(Error, Debug)]
pub enum BeatmapWorkerError {
    #[error("Failed to initialize beatmap worker: {0}")]
    #[allow(dead_code)]
    InitializationFailed(String),

    #[error("Beatmap processing failed: {0}")]
    #[allow(dead_code)]
    ProcessingFailed(String),

    #[error("Minacalc error: {0}")]
    #[allow(dead_code)]
    MinacalcError(String),

    #[error("Database error: {0}")]
    #[allow(dead_code)]
    DatabaseError(String),
}
