// src/errors/hardware_error.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HardwareError {
    #[error("Hardware not found: {0}")]
    NotFound(String),

    #[error("Hardware initialization failed: {0}")]
    InitializationError(String),

    #[error("Hardware communication error: {0}")]
    CommunicationError(String),

    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    #[error("Hardware is busy: {0}")]
    Busy(String),

    #[error("Hardware is in invalid state: {0}")]
    InvalidState(String),

    #[error("Operation timed out: {0}")]
    Timeout(String),

    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Other error: {0}")]
    Other(String),
}
