mod move_request;
mod pca9685_controller;

use crate::move_request::move_request::MoveRequest;
use crate::pca9685_controller::Pca9685Controller;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use pwm_pca9685::Channel;

/*// Constants for pulse widths
const MIN_PULSE: u16 = 150; // Approx 1 ms pulse width (minimum position)
const CENTER_PULSE: u16 = 375; // Approx 1.5 ms pulse width (center position)
const MAX_PULSE: u16 = 600; // Approx 2 ms pulse width (maximum position)*/

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
async fn move_servo_handler(
    body: web::Json<MoveRequest>,
    controller: web::Data<Pca9685Controller>,
) -> impl Responder {

    if body.angle > 90 {
        return HttpResponse::BadRequest().body("Invalid angle. Servo supports 0° to 90°.");
    }

    let pulse_width = angle_to_pulse(body.angle);

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

fn angle_to_pulse(angle: u16) -> u16 {
    let min_pulse = 246u32; // Counts for 1 ms pulse width
    let max_pulse = 492u32; // Counts for 2 ms pulse width

    // Limit angle to 0 - 90 degrees
    let angle = if angle > 90 { 90 } else { angle };

    let pulse_width = min_pulse + ((max_pulse - min_pulse) * angle as u32) / 90u32;
    pulse_width as u16
}


