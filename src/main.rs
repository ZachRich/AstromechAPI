// main.rs
mod move_request;
mod pca9685_controller;
mod servo_info;
mod servo_manager;
mod audio_manager;

use crate::move_request::move_request::MoveRequest;
use crate::servo_info::ServoInfo;
use crate::servo_manager::ServoManager;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use pwm_pca9685::Address;
use serde::Deserialize;
use std::fs;
use crate::audio_manager::AudioManager;

#[derive(Deserialize)]
struct Config {
    controllers: Vec<ControllerConfig>,
    servos: Vec<ServoInfo>,
}

#[derive(Deserialize)]
struct ControllerConfig {
    id: String,
    address: String, // Hex string like "0x40" or "default"
}

#[derive(Deserialize)]
struct PlayAudioRequest {
    filename: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load configuration
    let config_data =
        fs::read_to_string("servo_config.json").expect("Failed to read config.json");
    let config: Config = serde_json::from_str(&config_data).expect("Invalid servo_config.json");

    // Initialize controllers
    let controllers = config
        .controllers
        .into_iter()
        .map(|conf| {
            let address = match conf.address.as_str() {
                "default" => Address::default(),
                addr_str => {
                    let addr = u8::from_str_radix(&addr_str.trim_start_matches("0x"), 16)
                        .expect("Invalid address format");
                    Address::from(addr)
                }
            };
            let controller = pca9685_controller::Pca9685Controller::new(address);
            (conf.id, controller)
        })
        .collect();

    // Create the ServoManager
    let servo_manager = ServoManager::new(controllers, config.servos);

    // Set up the Actix server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(servo_manager.clone()))
            .route("/servos", web::get().to(get_available_servos_handler))
            .route("/servos/move", web::post().to(move_servo_handler))
    })
    .bind("0.0.0.0:3030")?
    .run()
    .await
}

// Handler function to get all available servos
async fn get_available_servos_handler(servo_manager: web::Data<ServoManager>) -> impl Responder {
    let servos = servo_manager.get_available_servos();
    HttpResponse::Ok().json(servos)
}

// Handler function to move a servo
async fn move_servo_handler(
    body: web::Json<MoveRequest>,
    servo_manager: web::Data<ServoManager>,
) -> impl Responder {
    let angle = body.angle;
    if angle > 90 {
        return HttpResponse::BadRequest().body("Invalid angle. Servo supports 0° to 90°.");
    }

    match servo_manager.move_servo(&body.servo_name, angle).await {
        Ok(_) => HttpResponse::Ok().json("Servo moved"),
        Err(err) => HttpResponse::BadRequest().body(err),
    }
}

async fn play_audio_handler(
    body: web::Json<PlayAudioRequest>,
    audio_manager: web::Data<AudioManager>,
) -> impl Responder {
    let filename = &body.filename;
    let file_path = format!("audio/{}", filename);
    // Call the play_audio method
    if let Err(e) = audio_manager.play_audio(&file_path) {
        eprintln!("Error playing audio: {}", e);
        return HttpResponse::InternalServerError().body("Failed to play audio");
    }
    HttpResponse::Ok().body("Audio playback started")
}

