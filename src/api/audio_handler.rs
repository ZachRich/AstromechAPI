// src/api/audio_handlers.rs
use crate::managers::audio_manager::AudioManager;
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct PlayAudioRequest {
    filename: String,
}

pub async fn list_audio_files(audio_manager: web::Data<AudioManager>) -> impl Responder {
    match audio_manager.list_audio_files().await {
        Ok(files) => HttpResponse::Ok().json(files),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn play_audio(
    audio_manager: web::Data<AudioManager>,
    req: web::Json<PlayAudioRequest>,
) -> impl Responder {
    match audio_manager.play_audio(&req.filename).await {
        Ok(id) => HttpResponse::Ok().json(serde_json::json!({
            "id": id,
            "message": "Audio playback started"
        })),
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}

pub async fn stop_audio(
    audio_manager: web::Data<AudioManager>,
    path: web::Path<Uuid>,
) -> impl Responder {
    match audio_manager.stop_audio(&path).await {
        Ok(_) => HttpResponse::Ok().json("Audio stopped"),
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}

pub async fn stop_all_audio(audio_manager: web::Data<AudioManager>) -> impl Responder {
    match audio_manager.stop_all().await {
        Ok(_) => HttpResponse::Ok().json("All audio stopped"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn get_audio_status(
    audio_manager: web::Data<AudioManager>,
    path: web::Path<Uuid>,
) -> impl Responder {
    match audio_manager.get_status(&path).await {
        Some(status) => HttpResponse::Ok().json(status),
        None => HttpResponse::NotFound().body("Playback not found"),
    }
}
