use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct AudioConfig {
    pub audio_directory: String,
    #[serde(default = "default_buffer_size")]
    pub buffer_size: u32,
    #[serde(default = "default_volume")]
    pub volume: f32,
}

fn default_buffer_size() -> u32 {
    32_768
}

fn default_volume() -> f32 {
    1.0
}
