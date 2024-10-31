// servo_manager.rs
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::pca9685_controller::Pca9685Controller;
use crate::servo_info::ServoInfo;
use pwm_pca9685::Channel;

const MIN_PULSE: u16 = 246; // Counts for 1 ms pulse width
const MAX_PULSE: u16 = 492; // Counts for 2 ms pulse width

#[derive(Clone)]
pub struct ServoManager {
    controllers: Arc<Mutex<HashMap<String, Arc<Pca9685Controller>>>>, // Map of controller IDs to controllers
    servos: Arc<Mutex<HashMap<String, ServoInfo>>>, // Map of servo names to ServoInfo
}

impl ServoManager {
    pub fn new(controllers: Vec<(String, Pca9685Controller)>, servos: Vec<ServoInfo>) -> Self {
        let controllers_map = controllers
            .into_iter()
            .map(|(id, controller)| (id, Arc::new(controller)))
            .collect();
        let servos_map = servos
            .into_iter()
            .map(|servo| (servo.name.clone(), servo))
            .collect();
        ServoManager {
            controllers: Arc::new(Mutex::new(controllers_map)),
            servos: Arc::new(Mutex::new(servos_map)),
        }
    }

    pub fn get_available_servos(&self) -> Vec<ServoInfo> {
        let servos = self.servos.lock().unwrap();
        servos.values().cloned().collect()
    }

    pub fn get_servo_by_name(&self, name: &str) -> Option<ServoInfo> {
        let servos = self.servos.lock().unwrap();
        servos.get(name).cloned()
    }

    pub async fn move_servo(&self, name: &str, angle: u16) -> Result<(), String> {
        let servo_info = self
            .get_servo_by_name(name)
            .ok_or_else(|| format!("Servo '{}' not found", name))?;

        let controllers = self.controllers.lock().unwrap();
        let controller = controllers
            .get(&servo_info.controller_id)
            .ok_or_else(|| format!("Controller '{}' not found", servo_info.controller_id))?;

        let channel = match servo_info.channel {
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
            _ => return Err(format!("Invalid channel '{}'", servo_info.channel)),
        };

        let pulse_width = Self::angle_to_pulse(angle);

        controller.move_servo(channel, pulse_width).await;
        Ok(())
    }

    // Function to calculate pulse width counts from angle
    fn angle_to_pulse(angle: u16) -> u16 {
        let min_pulse = MIN_PULSE as u32;
        let max_pulse = MAX_PULSE as u32;
        let pulse_width = min_pulse + ((max_pulse - min_pulse) * angle as u32) / 90u32;
        pulse_width as u16
    }
}
