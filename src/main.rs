mod pca9685_controller;
mod move_request;

use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use pwm_pca9685::Channel;
use serde::Deserialize;
use crate::move_request::move_request::MoveRequest;
use crate::pca9685_controller::Pca9685Controller;

// Constants for pulse widths
const MIN_PULSE: u16 = 150; // Approx 1 ms pulse width (minimum position)
const CENTER_PULSE: u16 = 375; // Approx 1.5 ms pulse width (center position)
const MAX_PULSE: u16 = 600; // Approx 2 ms pulse width (maximum position)

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Set up the PCA9685 controller
    let controller = Pca9685Controller::new();

    // Set up the Actix server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(controller.clone()))
            .route("/move", web::post().to(move_servo_handler))
    })
        .bind("0.0.0.0:3030")?
        .run()
        .await
}

// Async handler function to move a servo based on the API request
async fn move_servo_handler(body: web::Json<MoveRequest>, controller: web::Data<Pca9685Controller>) -> impl Responder {
    let pulse_width = match body.position.as_str() {
        "min" => MIN_PULSE,
        "center" => CENTER_PULSE,
        "max" => MAX_PULSE,
        _ => return HttpResponse::BadRequest().body("Invalid position"),
    };

    let channel = match body.channel {
        0 => Channel::C0,
        1 => Channel::C1,
        2 => Channel::C2,
        3 => Channel::C3,
        4 => Channel::C4,
        _ => return HttpResponse::BadRequest().body("Invalid channel"),
    };

    controller.move_servo(channel, pulse_width).await;
    HttpResponse::Ok().json("Servo moved")
}