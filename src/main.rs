extern crate linux_embedded_hal as hal;
extern crate pwm_pca9685 as pca9685;

use std::thread;
use std::time::Duration;
use linux_embedded_hal::I2cdev;
use pwm_pca9685::{Address, Channel, Pca9685};

// Constants for pulse widths
const MIN_PULSE: u16 = 150; // Approx 1 ms pulse width (minimum position)
const CENTER_PULSE: u16 = 375; // Approx 1.5 ms pulse width (center position)
const MAX_PULSE: u16 = 600; // Approx 2 ms pulse width (maximum position)


fn main() {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let address = Address::default();
    let mut pwm = Pca9685::new(dev, address).unwrap();

    // This corresponds to a frequency of 60 Hz.
    pwm.set_prescale(100).unwrap();

    // It is necessary to enable the device.
    pwm.enable().unwrap();

    // Define the list of channels you want to test
    let channels_to_test = [
        Channel::C0,
        Channel::C1,
        Channel::C2,
        Channel::C3,
        Channel::C4
        // Add more channels as needed
    ];

    // Run the test on each channel in the list
    test_multiple_channels(&mut pwm, &channels_to_test);

    let _dev = pwm.destroy(); // Get the I2C device back
}

// Function to move multiple servos to min, center, and max positions
fn test_multiple_channels(pwm: &mut Pca9685<I2cdev>, channels: &[Channel]) {
    for &channel in channels {
        println!("Testing Channel {:?}:", channel);

        // Move to minimum position
        move_servo(pwm, channel, MIN_PULSE);
        println!("Channel {:?} - Minimum position", channel);
        thread::sleep(Duration::from_secs(2));

        // Move to center position
        move_servo(pwm, channel, CENTER_PULSE);
        println!("Channel {:?} - Center position", channel);
        thread::sleep(Duration::from_secs(2));

        // Move to maximum position
        move_servo(pwm, channel, MAX_PULSE);
        println!("Channel {:?} - Maximum position", channel);
        thread::sleep(Duration::from_secs(2));
    }
}

// Helper function to move servo to a specific pulse width
fn move_servo(pwm: &mut Pca9685<hal::I2cdev>, channel: Channel, pulse_width: u16) {
    pwm.set_channel_on(channel, 0).unwrap();
    pwm.set_channel_off(channel, pulse_width).unwrap();
}

