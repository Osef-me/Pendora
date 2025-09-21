//! Pendora - A library for processing osu! beatmaps
//! 
//! This library provides functionality for:
//! - Processing osu! beatmaps and calculating ratings
//! - Converting between different data formats
//! - Managing beatmap workers and processing pipelines

pub mod core;
pub mod api;
pub mod utils;
pub mod errors;
pub mod config;


// Re-export config
pub use config::Config;

// Re-export errors
pub use errors::*;
