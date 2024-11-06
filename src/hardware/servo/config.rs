use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct Pca9685Config {
    pub id: String,
    pub i2c_address: String, // Hex string like "0x40"
    pub frequency: u16,      // PWM frequency in Hz
}

#[derive(Clone, Deserialize, Serialize)]
pub struct ServoConfig {
    pub name: String,
    pub controller_id: String, // References Pca9685Config id
    pub channel: u8,
    pub min_angle: f64,
    pub max_angle: f64,
    pub min_pulse: u16,
    pub max_pulse: u16,
    #[serde(default)]
    pub description: Option<String>,
}
