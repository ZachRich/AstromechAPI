extern crate linux_embedded_hal as hal;
extern crate pwm_pca9685 as pca9685;

use linux_embedded_hal::I2cdev;
use pwm_pca9685::{Address, Channel, Pca9685};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};

// Constants for pulse widths
const MIN_PULSE: u16 = 150; // Approx 1 ms pulse width (minimum position)
const CENTER_PULSE: u16 = 375; // Approx 1.5 ms pulse width (center position)
const MAX_PULSE: u16 = 600; // Approx 2 ms pulse width (maximum position)


#[tokio::main]
async fn main() {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let address = Address::default();
    let mut pwm = Pca9685::new(dev, address).unwrap();

    // This corresponds to a frequency of 60 Hz.
    pwm.set_prescale(100).unwrap();

    // It is necessary to enable the device.
    pwm.enable().unwrap();

    // Use an Arc<Mutex<_>> to share the PCA9685 instance across tasks
    let pwm = Arc::new(Mutex::new(pwm));

    // Define the list of channels you want to test
    let channels_to_test = [
        Channel::C0,
        Channel::C1,
        Channel::C2,
        Channel::C3,
        Channel::C4
        // Add more channels as needed
    ];

    // Spawn tasks for each channel to run concurrently
    let mut tasks = Vec::new();
    for &channel in &channels_to_test {
        let pwm = pwm.clone();
        tasks.push(tokio::spawn(async move {
            test_channel(pwm, channel).await;
        }));
    }

    // Wait for all tasks to complete
    for task in tasks {
        task.await.unwrap();
    }

}

// Async helper function to move a servo to a specific pulse width on a given channel
async fn move_servo(pwm: Arc<Mutex<Pca9685<hal::I2cdev>>>, channel: Channel, pulse_width: u16) {
    let mut pwm = pwm.lock().await;
    pwm.set_channel_on(channel, 0).unwrap();
    pwm.set_channel_off(channel, pulse_width).unwrap();
}

// Async function to test one channel
async fn test_channel(pwm: Arc<Mutex<Pca9685<hal::I2cdev>>>, channel: Channel) {
    println!("Testing Channel {:?}:", channel);
    for _i in 1..10 {
        // Move to minimum position
        move_servo(pwm.clone(), channel, MIN_PULSE).await;
        println!("Channel {:?} - Minimum position", channel);
        sleep(Duration::from_secs(2)).await;

        // Move to center position
        move_servo(pwm.clone(), channel, CENTER_PULSE).await;
        println!("Channel {:?} - Center position", channel);
        sleep(Duration::from_secs(2)).await;

        // Move to maximum position
        move_servo(pwm.clone(), channel, MAX_PULSE).await;
        println!("Channel {:?} - Maximum position", channel);
        sleep(Duration::from_secs(2)).await;
    }
}

