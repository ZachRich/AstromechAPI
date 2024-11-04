// Updated src/hardware/servo/pca9685.rs to implement Clone
use crate::errors::hardware_error::HardwareError;
use linux_embedded_hal::I2cdev;
use pwm_pca9685::{Address, Channel, Pca9685};
use std::sync::Arc;
use tokio::sync::Mutex;
use super::config::{Pca9685Config, ServoConfig};
use std::collections::HashMap;

#[derive(Clone)]
pub struct Pca9685Controller  {
    config: Pca9685Config,
    device: Arc<Mutex<Pca9685<I2cdev>>>,
    servos: Arc<Mutex<HashMap<String, ServoConfig>>>,
}

impl Pca9685Controller {
    pub fn new(config: Pca9685Config) -> Result<Self, HardwareError> {
        let device = Self::initialize_controller(&config)?;

        Ok(Self {
            config,
            device: Arc::new(Mutex::new(device)),
            servos: Arc::new(Mutex::new(HashMap::new())),
        })
    }

    fn initialize_controller(config: &Pca9685Config) -> Result<Pca9685<I2cdev>, HardwareError> {
        let address = Self::parse_address(&config.i2c_address)?;
        let i2c = I2cdev::new("/dev/i2c-1").map_err(|e| {
            HardwareError::InitializationError(format!("Failed to open I2C device: {:?}", e))
        })?;

        let mut device = Pca9685::new(i2c, address).map_err(|e| {
            HardwareError::InitializationError(format!("Failed to initialize PCA9685: {}", e))
        })?;

        // Set frequency and enable the device
        device.set_prescale(config.frequency as u8).map_err(|e| {
            HardwareError::InitializationError(format!("Failed to set frequency: {}", e))
        })?;

        device.enable().map_err(|e| {
            HardwareError::InitializationError(format!("Failed to enable device: {}", e))
        })?;

        Ok(device)
    }

    fn parse_address(addr_str: &str) -> Result<Address, HardwareError> {
        if addr_str == "default" {
            return Ok(Address::default());
        }

        let addr = u8::from_str_radix(addr_str.trim_start_matches("0x"), 16).map_err(|_| {
            HardwareError::InvalidParameter(format!("Invalid I2C address: {}", addr_str))
        })?;

        Ok(Address::from(addr))
    }

    pub async fn add_servo(&self, config: ServoConfig) -> Result<(), HardwareError> {
        let mut servos = self.servos.lock().await;
        servos.insert(config.name.clone(), config);
        Ok(())
    }

    pub async fn move_servo(&self, servo_name: &str, angle: f64) -> Result<(), HardwareError> {
        let servos = self.servos.lock().await;
        let servo = servos
            .get(servo_name)
            .ok_or_else(|| HardwareError::NotFound(format!("Servo '{}' not found", servo_name)))?;

        // Validate angle
        if angle < servo.min_angle || angle > servo.max_angle {
            return Err(HardwareError::InvalidParameter(format!(
                "Angle {} out of range [{}, {}]",
                angle, servo.min_angle, servo.max_angle
            )));
        }

        let pulse = self.angle_to_pulse(angle, servo);
        let channel = Self::channel_from_number(servo.channel)?;

        let mut device = self.device.lock().await;
        device.set_channel_on(channel, 0).map_err(|e| {
            HardwareError::CommunicationError(format!("Failed to set channel on: {}", e))
        })?;

        device.set_channel_off(channel, pulse).map_err(|e| {
            HardwareError::CommunicationError(format!("Failed to set channel off: {}", e))
        })?;

        Ok(())
    }

    fn angle_to_pulse(&self, angle: f64, servo: &ServoConfig) -> u16 {
        let min_pulse = servo.min_pulse as f64;
        let max_pulse = servo.max_pulse as f64;
        let min_angle = servo.min_angle;
        let max_angle = servo.max_angle;

        let pulse =
            min_pulse + (max_pulse - min_pulse) * (angle - min_angle) / (max_angle - min_angle);
        pulse as u16
    }

    fn channel_from_number(channel: u8) -> Result<Channel, HardwareError> {
        match channel {
            0 => Ok(Channel::C0),
            1 => Ok(Channel::C1),
            2 => Ok(Channel::C2),
            3 => Ok(Channel::C3),
            4 => Ok(Channel::C4),
            5 => Ok(Channel::C5),
            6 => Ok(Channel::C6),
            7 => Ok(Channel::C7),
            8 => Ok(Channel::C8),
            9 => Ok(Channel::C9),
            10 => Ok(Channel::C10),
            11 => Ok(Channel::C11),
            12 => Ok(Channel::C12),
            13 => Ok(Channel::C13),
            14 => Ok(Channel::C14),
            15 => Ok(Channel::C15),
            _ => Err(HardwareError::InvalidParameter(format!(
                "Invalid channel number: {}",
                channel
            ))),
        }
    }

    pub async fn get_servos(&self) -> HashMap<String, ServoConfig> {
        self.servos.lock().await.clone()
    }

    pub fn get_config(&self) -> &Pca9685Config {
        &self.config
    }
}
