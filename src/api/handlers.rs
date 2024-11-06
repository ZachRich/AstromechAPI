use crate::api::command::Command;
use crate::managers::routine_manager::RoutineManager;
use crate::managers::servo_manager::ServoManager;
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct MoveServoRequest {
    pub angle: f64,
}

pub async fn list_controllers(manager: web::Data<ServoManager>) -> impl Responder {
    let controllers = manager.list_controllers().await;
    HttpResponse::Ok().json(controllers)
}

pub async fn list_servos(manager: web::Data<ServoManager>) -> impl Responder {
    let servos = manager.list_servos().await;
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
#[derive(Deserialize)]
pub struct RoutineRequest {
    pub commands: Vec<Command>,
}

pub struct RoutineHandler {
    routine_manager: Arc<RoutineManager>,
}

impl RoutineHandler {
    pub fn new(routine_manager: Arc<RoutineManager>) -> Self {
        Self { routine_manager }
    }

    pub async fn execute_routine(&self, req: web::Json<RoutineRequest>) -> impl Responder {
        let commands = req.into_inner().commands;
        let routine_manager = Arc::clone(&self.routine_manager);

        // Execute routine asynchronously through the RoutineManager
        tokio::spawn(async move {
            routine_manager.execute_routine(commands).await;
        });

        HttpResponse::Ok().json("Routine started")
    }
}
