use crate::api::handlers::{list_controllers, list_servos, move_servo};
use actix_web::web;

// src/api/routes.rs
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/controllers", web::get().to(list_controllers))
            .route("/servos", web::get().to(list_servos))
            .route("/servos/{name}/move", web::post().to(move_servo)),
    );
}
