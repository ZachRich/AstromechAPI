
use serde::Serialize;
use serde::Deserialize;

#[derive(Serialize, Deserialize, Clone)]
pub struct ServoInfo {
    pub channel: u8,
    pub name: String,
    // Add more fields if necessary, like model, position, etc.
}
