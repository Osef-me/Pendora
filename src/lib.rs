//! Pendora - A library for processing osu! beatmaps
//!
//! This library provides functionality for:
//! - Processing osu! beatmaps and calculating ratings
//! - Converting between different data formats
//! - Managing beatmap workers and processing pipelines

pub mod api;
pub mod config;
pub mod core;
pub mod errors;
pub mod utils;

// Re-export config
pub use config::Config;

// Re-export errors
pub use errors::*;
