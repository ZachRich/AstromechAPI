use rppal::i2c::I2c;

#[allow(dead_code)]
pub struct I2cManager {
    i2c: I2c,
}

impl I2cManager {

    pub fn new() -> Self {
        let i2c = I2c::new().expect("Failed to create I2C instance");
        I2cManager { i2c }
    }

    pub fn list_servos(&self) -> String {
        // The PCA9685 can control up to 16 servos (0-15)
        let servos: Vec<u8> = (0..16).collect(); // Collect available servo channels

        servos.iter()
            .map(|&s| s.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    }

}
