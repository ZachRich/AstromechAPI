use std::fs;
use linux_embedded_hal::I2cdev;
use pwm_pca9685::{Address, Channel, Pca9685};
use std::sync::Arc;
use std::sync::Mutex;
use tokio::task;
use crate::servo_info::ServoInfo;

#[derive(Clone)]
pub struct Pca9685Controller {
    pwm: Arc<Mutex<Pca9685<I2cdev>>>,
    pub servos: Arc<Mutex<Vec<ServoInfo>>>,
}

impl Pca9685Controller {
    pub fn new() -> Self {
        let pwm = Pca9685Controller::setup_pca9685();

        // Load servos from configuration file
        let servo_data = fs::read_to_string("src/servo_config.json").expect("Failed to read servos.json");
        let servos: Vec<ServoInfo> = serde_json::from_str(&servo_data).expect("Invalid JSON");

        Pca9685Controller {
            pwm: Arc::new(Mutex::new(pwm)),
            servos: Arc::new(Mutex::new(servos)),
        }
    }

    fn setup_pca9685() -> Pca9685<I2cdev> {
        let dev = I2cdev::new("/dev/i2c-1").unwrap();
        let address = Address::default();
        let mut pwm = Pca9685::new(dev, address).unwrap();

        // Set the frequency to 60 Hz
        pwm.set_prescale(100).unwrap();

        // Enable the device
        pwm.enable().unwrap();

        pwm
    }

    pub fn get_servo_by_name(&self, name: &str) -> Option<ServoInfo> {
        let servos = self.servos.lock().unwrap();
        servos.iter().find(|s| s.name == name).cloned()
    }

    pub fn get_available_servos(&self) -> Vec<ServoInfo> {
        let servos = self.servos.lock().unwrap();
        servos.clone()
    }

    pub async fn move_servo(&self, channel: Channel, pulse_width: u16) {
        let pwm = self.pwm.clone();
        task::spawn(async move {
            let mut pwm = pwm.lock().unwrap();
            pwm.set_channel_on(channel, 0).unwrap();
            pwm.set_channel_off(channel, pulse_width).unwrap();
        })
        .await
        .unwrap();
    }
}
