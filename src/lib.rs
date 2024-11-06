pub mod api;
pub mod config;
pub mod errors;
pub mod hardware;
pub mod managers;

// Explicitly re-export AudioManager if needed
pub use crate::managers::audio_manager::AudioManager;
