// src/main.rs
use actix_web::{middleware, web, App, HttpServer};
use log::{error, info, LevelFilter};
use serde::Deserialize;
use std::fs;
use std::sync::Arc;

mod api;
mod config;
mod errors;
mod hardware;
mod managers;
mod traits;

use crate::errors::hardware_error::HardwareError;
use crate::hardware::servo::config::{Pca9685Config, ServoConfig};
use crate::managers::servo_manager::ServoManager;
use crate::managers::audio_manager::AudioManager;
use crate::hardware::audio::config::AudioConfig;

#[derive(Deserialize)]
struct Config {
    controllers: Vec<Pca9685Config>,
    servos: Vec<ServoConfig>,
    server: ServerConfig,
    audio: AudioConfig,
}

#[derive(Deserialize)]
struct ServerConfig {
    host: String,
    port: u16,
    log_level: String,
}

async fn initialize_hardware(
    config: &Config,
    servo_manager_data: &web::Data<ServoManager>,
) -> Result<(), HardwareError> {
    // Initialize PCA9685 controllers
    for controller_config in &config.controllers {
        info!("Initializing controller: {}", controller_config.id);
        servo_manager_data
            .initialize_controller(controller_config.clone())
            .await?;
    }

    // Initialize servos
    for servo_config in &config.servos {
        info!("Initializing servo: {}", servo_config.name);
        servo_manager_data.add_servo(servo_config.clone()).await?;
    }

    Ok(())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load configuration
    let config = load_config()?;

    // Setup logging
    setup_logging(&config.server.log_level);
    info!("Starting Astromech control system...");

    // Check I2C setup
    if let Err(e) = check_i2c_setup().await {
        error!("I2C setup check failed: {}", e);
        return Err(std::io::Error::new(std::io::ErrorKind::Other, e));
    }

    // Initialize managers
    let servo_manager = ServoManager::new();
    let servo_manager_data = web::Data::new(servo_manager);

    // Initialize hardware using the wrapped servo manager
    if let Err(e) = initialize_hardware(&config, &servo_manager_data).await {
        error!("Failed to initialize hardware: {}", e);
        return Err(std::io::Error::new(std::io::ErrorKind::Other, e));
    }

    // Initialize audio manager
    let audio_manager = AudioManager::new(config.audio.clone())
        .map_err(|e| {
            error!("Failed to initialize audio manager: {}", e);
            std::io::Error::new(std::io::ErrorKind::Other, e)
        })?;
    let audio_manager_data = web::Data::new(audio_manager);

    info!("Hardware initialization complete");

    // Create server bind address
    let bind_addr = format!("{}:{}", config.server.host, config.server.port);
    info!("Starting server on {}", bind_addr);

    // Set up and start the HTTP server
    let server = HttpServer::new(move || {
        App::new()
            .app_data(servo_manager_data.clone())
            .app_data(audio_manager_data.clone())
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .wrap(
                middleware::DefaultHeaders::new()
                    .add(("X-Version", "1.0"))
                    .add(("X-Server", "Astromech-Control"))
            )
            .configure(api::routes::configure_routes)
    })
        .bind(&bind_addr)?
        .workers(2)
        .shutdown_timeout(30);

    // Run the server
    info!("Server starting...");
    server.run().await
}

fn load_config() -> std::io::Result<Config> {
    let config_path = std::env::var("ASTROMECH_CONFIG")
        .unwrap_or_else(|_| "astromech_config.json".to_string());

    let config_data = fs::read_to_string(&config_path)?;

    serde_json::from_str(&config_data)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
}

fn setup_logging(log_level: &str) {
    let level = match log_level.to_lowercase().as_str() {
        "debug" => LevelFilter::Debug,
        "info" => LevelFilter::Info,
        "warn" => LevelFilter::Warn,
        "error" => LevelFilter::Error,
        _ => LevelFilter::Info,
    };

    env_logger::Builder::new()
        .filter_level(level)
        .format_timestamp_millis()
        .init();
}

async fn check_i2c_setup() -> Result<(), HardwareError> {
    let i2c_path = "/dev/i2c-1";

    // Check if I2C device exists
    if !std::path::Path::new(i2c_path).exists() {
        error!("I2C device not found. Please ensure I2C is enabled:");
        error!("1. Run 'sudo raspi-config'");
        error!("2. Go to Interface Options -> I2C -> Enable");
        error!("3. Reboot the system");
        return Err(HardwareError::InitializationError(
            "I2C device not found".to_string(),
        ));
    }

    // Try to open the device to check access
    match fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(i2c_path)
    {
        Ok(_) => {
            info!("I2C device access verified");
            Ok(())
        },
        Err(e) => {
            error!("Cannot access I2C device. Please run:");
            error!("sudo chown root:gpio {}", i2c_path);
            error!("sudo chmod 666 {}", i2c_path);
            Err(HardwareError::InitializationError(
                format!("Cannot access I2C device: {}", e)
            ))
        }
    }
}