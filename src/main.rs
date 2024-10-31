mod move_request;
mod pca9685_controller;
mod servo_info;

use crate::move_request::move_request::MoveRequest;
use crate::pca9685_controller::Pca9685Controller;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use pwm_pca9685::Channel;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Set up the PCA9685 controller
    let controller = Pca9685Controller::new();

    // Set up the Actix server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(controller.clone()))
            .route("/servos", web::get().to(get_available_servos_handler))
            .route("/servos/move", web::post().to(move_servo_handler))
    })
    .bind("0.0.0.0:3030")?
    .run()
    .await
}

// Handler function to get all available servos
async fn get_available_servos_handler(
    controller: web::Data<Pca9685Controller>,
) -> impl Responder {
    let servos = controller.get_available_servos();
    HttpResponse::Ok().json(servos)
}

async fn move_servo_handler(
    body: web::Json<MoveRequest>,
    controller: web::Data<Pca9685Controller>,
) -> impl Responder {
    let angle = body.angle;
    if angle > 90 {
        return HttpResponse::BadRequest().body("Invalid angle. Servo supports 0° to 90°.");
    }

    // Look up the servo by name
    let servo_name = &body.servo_name;
    let servo_info = controller.get_servo_by_name(servo_name);
    let servo_info = match servo_info {
        Some(info) => info,
        None => return HttpResponse::BadRequest().body("Servo not found"),
    };

    let pulse_width = angle_to_pulse(angle);

    // Convert the channel number to `Channel` enum
    let channel = match servo_info.channel {
        0 => Channel::C0,
        1 => Channel::C1,
        2 => Channel::C2,
        3 => Channel::C3,
        4 => Channel::C4,
        // Add more channels as needed
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


