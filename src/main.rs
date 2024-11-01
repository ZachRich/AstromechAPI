// src/main.rs
mod move_request;
mod pca9685_controller;
mod servo_info;
mod servo_manager;
mod audio_manager;

use crate::pca9685_controller::Pca9685Controller;
use crate::servo_info::ServoInfo;
use crate::servo_manager::ServoManager;

use crate::audio_manager::AudioManager;
use crate::move_request::move_request::MoveRequest;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use pwm_pca9685::Address;
use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
struct Config {
    controllers: Vec<ControllerConfig>,
    servos: Vec<ServoInfo>,
}

#[derive(Deserialize)]
struct ControllerConfig {
    id: String,
    address: String,
}

#[derive(Deserialize)]
struct PlayAudioRequest {
    filename: String,
}

const MIN_PULSE: u16 = 246; // Counts for 1 ms pulse width
const MAX_PULSE: u16 = 492; // Counts for 2 ms pulse width

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load configuration
    let config_data = fs::read_to_string("servo_config.json").expect("Failed to read servo_config.json");
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
            let controller = Pca9685Controller::new(address);
            (conf.id, controller)
        })
        .collect();

    // Create the ServoManager
    let servo_manager = ServoManager::new(controllers, config.servos);
    // Wrap ServoManager in web::Data
    let servo_manager_data = web::Data::new(servo_manager);

    HttpServer::new(move || {
        let servo_manager = servo_manager_data.clone();
        let audio_manager = AudioManager::new();
        App::new()
            .app_data(servo_manager)
            .app_data(web::Data::new(audio_manager))
            .route("/servos", web::get().to(get_available_servos_handler))
            .route("/servos/move", web::post().to(move_servo_handler))
            .route("/audio/play", web::post().to(play_audio_handler))
    })
        .bind("0.0.0.0:3030")?
        .run()
        .await
}

// Handler function to get all available servos
async fn get_available_servos_handler(
    servo_manager: web::Data<ServoManager>,
) -> impl Responder {
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

    let pulse_width = angle_to_pulse(angle);

    match servo_manager.move_servo(&body.servo_name, pulse_width).await {
        Ok(_) => HttpResponse::Ok().json("Servo moved"),
        Err(err) => HttpResponse::BadRequest().body(err),
    }
}

// Function to calculate pulse width counts from angle
fn angle_to_pulse(angle: u16) -> u16 {
    let min_pulse = MIN_PULSE as u32;
    let max_pulse = MAX_PULSE as u32;
    let pulse_width = min_pulse + ((max_pulse - min_pulse) * angle as u32) / 90u32;
    pulse_width as u16
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



