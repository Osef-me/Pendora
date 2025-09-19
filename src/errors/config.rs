use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Failed to load environment variables: {0}")]
    EnvLoadError(#[from] dotenvy::Error),

    #[error("Missing required environment variable: {0}")]
    MissingVariable(String),
}
