// main.rs
use actix_web::{middleware, web, App, HttpServer};
use log::{error, info, LevelFilter};
use serde::Deserialize;
use std::fs;
use tokio::signal;

mod api;
mod config;
mod errors;
mod hardware;
mod managers;
mod traits;

use crate::hardware::servo::config::{Pca9685Config, ServoConfig};
use crate::managers::servo_manager::ServoManager;
use crate::errors::hardware_error::HardwareError;

#[derive(Deserialize)]
struct Config {
    controllers: Vec<Pca9685Config>,
    servos: Vec<ServoConfig>,
    server: ServerConfig,
}

#[derive(Deserialize)]
struct ServerConfig {
    host: String,
    port: u16,
    log_level: String,
}

fn load_config() -> std::io::Result<Config> {
    let config_path = std::env::var("ASTROMECH_CONFIG")
        .unwrap_or_else(|_| "astromech_config.json".to_string());

    let config_data = fs::read_to_string(&config_path)?;

    serde_json::from_str(&config_data)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
}

async fn initialize_hardware(
    config: &Config,
    servo_manager: &ServoManager,
) -> Result<(), HardwareError> {
    // Initialize PCA9685 controllers
    for controller_config in &config.controllers {
        info!("Initializing controller: {}", controller_config.id);
        servo_manager
            .add_controller(controller_config.clone())
            .await?;
    }

    // Initialize servos
    for servo_config in &config.servos {
        info!("Initializing servo: {}", servo_config.name);
        servo_manager.add_servo(servo_config.clone()).await?;
    }

    Ok(())
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load configuration
    let config = load_config()?;

    // Setup logging
    setup_logging(&config.server.log_level);
    info!("Starting Astromech control system...");

    // Initialize servo manager
    let servo_manager = ServoManager::new();

    // Initialize hardware
    if let Err(e) = initialize_hardware(&config, &servo_manager).await {
        error!("Failed to initialize hardware: {}", e);
        return Err(std::io::Error::new(std::io::ErrorKind::Other, e));
    }

    info!("Hardware initialization complete");

    // Create server bind address
    let bind_addr = format!("{}:{}", config.server.host, config.server.port);
    info!("Starting server on {}", bind_addr);

    // Set up the HTTP server with graceful shutdown
    let servo_manager = web::Data::new(servo_manager);

    let server = HttpServer::new(move || {
        App::new()
            .app_data(servo_manager.clone())
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .configure(api::routes::configure_routes)
    })
        .bind(&bind_addr)?
        .workers(2)
        .shutdown_timeout(30); // 30 seconds shutdown timeout

    // Start the server and wait for shutdown signal
    let running_server = server.run();

    // Create a future that completes when Ctrl+C is pressed
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to listen for ctrl+c");
        info!("Received shutdown signal, starting graceful shutdown...");
    };

    // Run the server until Ctrl+C is received
    tokio::select! {
        _ = ctrl_c => {
            info!("Shutting down server...");
        }
        result = running_server => {
            if let Err(e) = result {
                error!("Server error: {}", e);
            }
        }
    }

    info!("Server shutdown complete");
    Ok(())
}