// pca9685_controller.rs
use linux_embedded_hal::I2cdev;
use pwm_pca9685::{Address, Channel, Pca9685};
use std::sync::{Arc, Mutex};
use tokio::task;

#[derive(Clone)]
pub struct Pca9685Controller {
    pwm: Arc<Mutex<Pca9685<I2cdev>>>,
    pub address: Address,
}

impl Pca9685Controller {
    pub fn new(address: Address) -> Self {
        let pwm = Pca9685Controller::setup_pca9685(address.clone());
        Pca9685Controller {
            pwm: Arc::new(Mutex::new(pwm)),
            address,
        }
    }

    fn setup_pca9685(address: Address) -> Pca9685<I2cdev> {
        let dev = I2cdev::new("/dev/i2c-1").unwrap();
        let mut pwm = Pca9685::new(dev, address).unwrap();

        // Set the frequency to 60 Hz
        pwm.set_prescale(100).unwrap();

        // Enable the device
        pwm.enable().unwrap();

        pwm
    }

    pub async fn move_servo(&self, channel: Channel, pulse_width: u16) {
        let pwm_clone = self.pwm.clone();
        task::spawn(async move {
            let mut pwm = pwm_clone.lock().unwrap();
            pwm.set_channel_on(channel, 0).unwrap();
            pwm.set_channel_off(channel, pulse_width).unwrap();
        })
        .await
        .unwrap();
    }
}
