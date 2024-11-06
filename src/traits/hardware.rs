use crate::errors::hardware_error::HardwareError;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[async_trait]
pub trait Hardware: Send + Sync + Debug {
    /// Get the unique identifier for this hardware
    fn id(&self) -> &str;

    /// Get the type of hardware (e.g., "servo", "motor", "led")
    fn hardware_type(&self) -> &str;

    /// Initialize the hardware
    async fn initialize(&self) -> Result<(), HardwareError>;

    /// Perform any cleanup necessary when shutting down
    async fn shutdown(&self) -> Result<(), HardwareError>;

    /// Check if the hardware is currently operational
    async fn is_operational(&self) -> bool;

    /// Get the current status of the hardware
    async fn get_status(&self) -> HardwareStatus;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HardwareStatus {
    Operational,
    Error(String),
    Initializing,
    ShuttingDown,
    Offline,
}
