use rppal::i2c::I2c;

pub struct I2cManager {
    i2c: I2c,
}

impl I2cManager {
    pub fn new() -> Self {
        let i2c = I2c::new().expect("Failed to create I2C instance");
        I2cManager { i2c }
    }

    pub fn list_servos(&self) -> Vec<u8> {
        // The PCA9685 can control up to 16 servos (0-15)
        (0..16).collect() // Return the available servo channels
    }

    pub fn set_servo_angle(&mut self, angle: u16) {
        // TODO
    }
}
