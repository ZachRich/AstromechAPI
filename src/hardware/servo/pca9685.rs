// src/hardware/servo/pca9685.rs
use crate::errors::hardware_error::HardwareError;
use linux_embedded_hal::I2cdev;
use pwm_pca9685::{Address, Channel, Pca9685};
use std::sync::Arc;
use tokio::sync::Mutex;
use log::info;
use crate::hardware::servo::Pca9685Config;

pub struct Pca9685Controller {
    config: Pca9685Config,
    device: Arc<Mutex<Pca9685<I2cdev>>>,
}

impl Pca9685Controller {
    pub fn new(config: Pca9685Config) -> Result<Self, HardwareError> {
        let i2c = I2cdev::new("/dev/i2c-1").map_err(|e| {
            HardwareError::InitializationError(format!("Failed to open I2C device: {:?}", e))
        })?;

        let address = if config.i2c_address == "default" {
            Address::default()
        } else {
            let addr = u8::from_str_radix(&config.i2c_address.trim_start_matches("0x"), 16)
                .map_err(|_| HardwareError::InvalidParameter(format!(
                    "Invalid I2C address: {}", config.i2c_address
                )))?;
            Address::from(addr)
        };

        let mut device = Pca9685::new(i2c, address).map_err(|e| {
            HardwareError::InitializationError(format!("Failed to initialize PCA9685: {}", e))
        })?;

        // Set frequency (usually 50Hz for servos)
        device.set_prescale(100).map_err(|e| {
            HardwareError::InitializationError(format!("Failed to set frequency: {}", e))
        })?;

        // Enable the device
        device.enable().map_err(|e| {
            HardwareError::InitializationError(format!("Failed to enable device: {}", e))
        })?;

        Ok(Self {
            config,
            device: Arc::new(Mutex::new(device)),
        })
    }

    pub async fn move_servo(&self, channel: Channel, pulse_width: u16) -> Result<(), HardwareError> {
        let mut device = self.device.lock().await;

        info!("Setting pulse width {} on channel {:?}", pulse_width, channel);

        device.set_channel_on(channel, 0).map_err(|e| {
            HardwareError::CommunicationError(format!("Failed to set channel on: {}", e))
        })?;

        device.set_channel_off(channel, pulse_width).map_err(|e| {
            HardwareError::CommunicationError(format!("Failed to set channel off: {}", e))
        })?;

        Ok(())
    }

    pub fn get_config(&self) -> &Pca9685Config {
        &self.config
    }
}