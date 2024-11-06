use crate::api::command::Command;
use crate::managers::{audio_manager::AudioManager, servo_manager::ServoManager};
use actix_web::web::Data;
use std::sync::Arc;
use std::time::Duration;

pub struct RoutineManager {
    servo_manager: Data<ServoManager>,
    audio_manager: Data<AudioManager>,
}

impl RoutineManager {
    pub fn new(servo_manager: Data<ServoManager>, audio_manager: Data<AudioManager>) -> Self {
        Self {
            servo_manager,
            audio_manager,
        }
    }

    pub async fn execute_routine(&self, commands: Vec<Command>) {
        for command in commands {
            match command {
                Command::MoveServo {
                    servo_name,
                    position,
                    duration: _duration,
                } => {
                    let servo_manager = Arc::clone(&self.servo_manager);
                    tokio::spawn(async move {
                        servo_manager
                            .move_servo(&servo_name, position)
                            .await
                            .unwrap();
                    });
                }
                Command::PlayAudio { file } => {
                    let audio_manager = Arc::clone(&self.audio_manager);
                    tokio::spawn(async move {
                        audio_manager.play_audio(&file).await.unwrap();
                    });
                }
                Command::Pause { duration } => {
                    tokio::time::sleep(Duration::from_millis(duration)).await;
                }
            }
        }
    }
}
