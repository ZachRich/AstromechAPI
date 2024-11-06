use crate::api::audio_handler;
use crate::api::audio_handler::get_duration;
use crate::api::handlers::{
    list_controllers, list_servos, move_servo, RoutineHandler, RoutineRequest,
};
use actix_web::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/controllers", web::get().to(list_controllers))
            .route("/servos", web::get().to(list_servos))
            .route("/servos/{name}/move", web::post().to(move_servo))
            .route(
                "/routine",
                web::post().to(
                    |routine_request: web::Json<RoutineRequest>,
                     routine_handler: web::Data<RoutineHandler>| async move {
                        routine_handler.execute_routine(routine_request).await
                    },
                ),
            )
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
