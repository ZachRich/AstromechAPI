pub mod api;
pub mod config;
pub mod errors;
pub mod hardware;
pub mod managers;
pub mod traits;

// Explicitly re-export AudioManager if needed
pub use crate::managers::audio_manager::AudioManager;
