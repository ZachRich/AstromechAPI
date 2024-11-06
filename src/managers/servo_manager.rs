use log::{error, info};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::errors::hardware_error::HardwareError;
use crate::hardware::servo::config::{Pca9685Config, ServoConfig};
use crate::hardware::servo::Pca9685Controller;
use pwm_pca9685::Channel;

#[derive(Clone)]
pub struct ServoManager {
    controllers: Arc<Mutex<HashMap<String, Arc<Pca9685Controller>>>>,
    servos: Arc<Mutex<HashMap<String, ServoConfig>>>,
}

impl ServoManager {
    pub fn new() -> Self {
        Self {
            controllers: Arc::new(Mutex::new(HashMap::new())),
            servos: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn initialize_controller(&self, config: Pca9685Config) -> Result<(), HardwareError> {
        let controller = Pca9685Controller::new(config.clone())?;
        let mut controllers = self.controllers.lock().await;
        controllers.insert(config.id.clone(), Arc::new(controller));
        Ok(())
    }

    pub async fn list_controllers(&self) -> HashMap<String, Pca9685Config> {
        let controllers = self.controllers.lock().await;
        controllers
            .iter()
            .map(|(id, controller)| (id.clone(), controller.get_config().clone()))
            .collect()
    }

    pub async fn add_servo(&self, config: ServoConfig) -> Result<(), HardwareError> {
        let controllers = self.controllers.lock().await;

        // Verify controller exists
        if !controllers.contains_key(&config.controller_id) {
            return Err(HardwareError::NotFound(format!(
                "Controller '{}' not found",
                config.controller_id
            )));
        }

        // Store servo config
        let mut servos = self.servos.lock().await;
        servos.insert(config.name.clone(), config);
        Ok(())
    }

    pub async fn move_servo(&self, name: &str, angle: f64) -> Result<(), HardwareError> {
        // Get servo config
        let servo_config = {
            let servos = self.servos.lock().await;
            servos
                .get(name)
                .cloned()
                .ok_or_else(|| HardwareError::NotFound(format!("Servo '{}' not found", name)))?
        };

        // Validate angle
        if angle < servo_config.min_angle || angle > servo_config.max_angle {
            return Err(HardwareError::InvalidParameter(format!(
                "Angle {} is outside valid range [{}, {}]",
                angle, servo_config.min_angle, servo_config.max_angle
            )));
        }

        // Get controller
        let controllers = self.controllers.lock().await;
        let controller = controllers
            .get(&servo_config.controller_id)
            .ok_or_else(|| {
                HardwareError::NotFound(format!(
                    "Controller '{}' not found",
                    servo_config.controller_id
                ))
            })?;

        // Convert angle to pulse width
        let pulse_width = self.angle_to_pulse(&servo_config, angle);

        // Get channel
        let channel = match servo_config.channel {
            0 => Channel::C0,
            1 => Channel::C1,
            2 => Channel::C2,
            3 => Channel::C3,
            4 => Channel::C4,
            5 => Channel::C5,
            6 => Channel::C6,
            7 => Channel::C7,
            8 => Channel::C8,
            9 => Channel::C9,
            10 => Channel::C10,
            11 => Channel::C11,
            12 => Channel::C12,
            13 => Channel::C13,
            14 => Channel::C14,
            15 => Channel::C15,
            _ => {
                return Err(HardwareError::InvalidParameter(format!(
                    "Invalid channel number: {}",
                    servo_config.channel
                )))
            }
        };

        // Move servo
        info!(
            "Moving servo '{}' to angle {} (pulse width {})",
            name, angle, pulse_width
        );
        controller.move_servo(channel, pulse_width).await?;

        Ok(())
    }

    fn angle_to_pulse(&self, servo: &ServoConfig, angle: f64) -> u16 {
        let angle_range = servo.max_angle - servo.min_angle;
        let pulse_range = servo.max_pulse - servo.min_pulse;

        let normalized_angle = angle - servo.min_angle;
        let pulse_width =
            servo.min_pulse as f64 + (normalized_angle * pulse_range as f64) / angle_range;

        pulse_width as u16
    }

    pub async fn get_servo_config(&self, name: &str) -> Option<ServoConfig> {
        let servos = self.servos.lock().await;
        servos.get(name).cloned()
    }

    pub async fn list_servos(&self) -> HashMap<String, ServoConfig> {
        let servos = self.servos.lock().await;
        servos.clone()
    }
}
