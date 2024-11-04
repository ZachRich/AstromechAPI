// src/api/handlers.rs
use crate::managers::servo_manager::ServoManager;
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct MoveServoRequest {
    pub angle: f64,
}

pub async fn list_controllers(manager: web::Data<ServoManager>) -> impl Responder {
    let controllers = manager.get_controllers().await;
    HttpResponse::Ok().json(controllers)
}

pub async fn list_servos(manager: web::Data<ServoManager>) -> impl Responder {
    let servos = manager.get_all_servos().await;
    HttpResponse::Ok().json(servos)
}

pub async fn move_servo(
    servo_name: web::Path<String>,
    req: web::Json<MoveServoRequest>,
    manager: web::Data<ServoManager>,
) -> impl Responder {
    match manager.move_servo(&servo_name, req.angle).await {
        Ok(_) => HttpResponse::Ok().json("Servo moved successfully"),
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}
