use crate::managers::servo_manager::ServoManager;
use crate::AudioManager;
use serde::Deserialize;
use std::sync::Arc;
use std::time::Duration;

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Command {
    MoveServo {
        servo_name: String,
        position: f64,
        duration: u64,
    },
    PlayAudio {
        file: String,
    },
    Pause {
        duration: u64,
    },
}

impl Command {
    pub async fn execute(
        &self,
        servo_manager: Arc<ServoManager>,
        audio_manager: Arc<AudioManager>,
    ) {
        match self {
            Command::MoveServo {
                servo_name,
                position,
                duration,
            } => {
                // Call the move_servo function on ServoManager
                if let Err(e) = servo_manager.move_servo(&servo_name, *position).await {
                    eprintln!("Error moving servo: {:?}", e);
                }
            }
            Command::PlayAudio { file } => {
                // Call the play_audio function on AudioManager
                if let Err(e) = audio_manager.play_audio(&file.clone()).await {
                    eprintln!("Error playing audio: {:?}", e);
                }
            }
            Command::Pause { duration } => {
                // Pause for the specified duration
                tokio::time::sleep(Duration::from_millis(*duration)).await;
            }
        }
    }
}
