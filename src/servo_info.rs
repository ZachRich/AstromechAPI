use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Clone)]
pub struct ServoInfo {
    pub controller_id: String, // Identifier for the controller
    pub channel: u8,
    pub name: String,
    // Add more fields if necessary, like model, position, etc.
}
