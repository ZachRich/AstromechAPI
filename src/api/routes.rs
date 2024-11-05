use crate::api::audio_handler;
use crate::api::audio_handler::get_duration;
use crate::api::handlers::{list_controllers, list_servos, move_servo};
use actix_web::web;

// src/api/routes.rs
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/controllers", web::get().to(list_controllers))
            .route("/servos", web::get().to(list_servos))
            .route("/servos/{name}/move", web::post().to(move_servo))
            .service(
                web::scope("/audio")
                    .route("", web::get().to(audio_handler::list_audio_files))
                    .route("/play", web::post().to(audio_handler::play_audio))
                    .route("/stop/{id}", web::post().to(audio_handler::stop_audio))
                    .route("/stop", web::post().to(audio_handler::stop_all_audio))
                    .route(
                        "/status/{id}",
                        web::get().to(audio_handler::get_audio_status),
                    )
                    .route("/duration", web::post().to(get_duration)),
            ),
    );
}
