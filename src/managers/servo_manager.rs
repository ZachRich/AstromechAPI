// src/managers/servo_manager.rs
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::errors::hardware_error::HardwareError;
use crate::hardware::servo::{Pca9685Controller, Pca9685Config, ServoConfig};

#[derive(Clone)]
pub struct ServoManager {
    controllers: Arc<Mutex<HashMap<String, Arc<Pca9685Controller>>>>,
}

impl ServoManager {
    pub fn new() -> Self {
        Self {
            controllers: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn add_controller(&self, config: Pca9685Config) -> Result<(), HardwareError> {
        let controller = Pca9685Controller::new(config.clone())?;
        let mut controllers = self.controllers.lock().await;
        controllers.insert(config.id.clone(), Arc::new(controller));
        Ok(())
    }

    pub async fn add_servo(&self, config: ServoConfig) -> Result<(), HardwareError> {
        let controllers = self.controllers.lock().await;
        let controller = controllers.get(&config.controller_id)
            .ok_or_else(|| HardwareError::NotFound(
                format!("Controller '{}' not found", config.controller_id)))?;

        controller.add_servo(config).await
    }

    pub async fn move_servo(&self, servo_name: &str, angle: f64) -> Result<(), HardwareError> {
        let controllers = self.controllers.lock().await;

        // Search through all controllers for the servo
        for controller in controllers.values() {
            let servos = controller.get_servos().await;
            if servos.contains_key(servo_name) {
                return controller.move_servo(servo_name, angle).await;
            }
        }

        Err(HardwareError::NotFound(format!("Servo '{}' not found", servo_name)))
    }

    pub async fn get_controllers(&self) -> Vec<Pca9685Config> {
        let controllers = self.controllers.lock().await;
        controllers.values()
            .map(|c| c.get_config().clone())
            .collect()
    }

    pub async fn get_all_servos(&self) -> HashMap<String, ServoConfig> {
        let mut all_servos = HashMap::new();
        let controllers = self.controllers.lock().await;

        for controller in controllers.values() {
            all_servos.extend(controller.get_servos().await);
        }

        all_servos
    }

    pub async fn get_servo(&self, name: &str) -> Option<ServoConfig> {
        let controllers = self.controllers.lock().await;

        for controller in controllers.values() {
            let servos = controller.get_servos().await;
            if let Some(servo) = servos.get(name) {
                return Some(servo.clone());
            }
        }

        None
    }
}