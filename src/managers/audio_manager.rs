// src/managers/audio_manager.rs
use crate::errors::hardware_error::HardwareError;
use crate::hardware::audio::config::AudioConfig;
use log::{error, info};
use serde::Serialize;
use std::collections::HashMap;
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::sync::Arc;
use tokio::process::Command;
use tokio::sync::Mutex;
use uuid::Uuid;

#[derive(Clone)]
pub struct AudioManager {
    config: AudioConfig,
    active_playbacks: Arc<Mutex<HashMap<Uuid, bool>>>,
}

#[derive(Serialize)]
pub struct AudioFile {
    name: String,
    path: String,
}

#[derive(Serialize)]
pub struct PlaybackStatus {
    id: Uuid,
    playing: bool,
    file_name: String,
}

impl AudioManager {
    pub fn new(config: AudioConfig) -> Result<Self, HardwareError> {
        Ok(Self {
            config,
            active_playbacks: Arc::new(Mutex::new(HashMap::new())),
        })
    }

    pub async fn list_audio_files(&self) -> Result<Vec<AudioFile>, HardwareError> {
        let audio_path = Path::new(&self.config.audio_directory);
        if !audio_path.exists() {
            return Err(HardwareError::NotFound(
                "Audio directory not found".to_string(),
            ));
        }

        let mut audio_files = Vec::new();

        for entry in fs::read_dir(audio_path)
            .map_err(|e| HardwareError::Other(format!("Failed to read audio directory: {}", e)))?
        {
            let entry = entry.map_err(|e| {
                HardwareError::Other(format!("Failed to read directory entry: {}", e))
            })?;

            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("mp3") {
                let name = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .ok_or_else(|| HardwareError::Other("Invalid file name".to_string()))?
                    .to_string();

                audio_files.push(AudioFile {
                    name: name.clone(),
                    path: format!("/audio/{}", name),
                });
            }
        }

        Ok(audio_files)
    }

    pub async fn play_audio(&self, filename: &str) -> Result<Uuid, HardwareError> {
        let full_path = Path::new(&self.config.audio_directory).join(filename);
        if !full_path.exists() {
            return Err(HardwareError::NotFound(format!(
                "Audio file not found: {}",
                filename
            )));
        }

        let id = Uuid::new_v4();
        let playbacks = Arc::clone(&self.active_playbacks);
        let path_string = full_path.to_string_lossy().to_string();
        let filename_string = filename.to_string();

        // Insert the playback before starting
        {
            let mut playbacks_lock = playbacks.lock().await;
            playbacks_lock.insert(id, true);
        }

        // Spawn a new tokio task to handle the playback
        tokio::spawn(async move {
            let result = Command::new("mpg123")
                .arg("-q") // Quiet mode
                .arg(&path_string)
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status()
                .await;

            let mut playbacks_lock = playbacks.lock().await;
            match result {
                Ok(status) => {
                    if !status.success() {
                        error!("Failed to play audio file: {}", filename_string);
                    }
                }
                Err(e) => {
                    error!("Error playing audio: {}", e);
                }
            }
            playbacks_lock.remove(&id);
        });

        info!("Started playing audio file: {}", filename);
        Ok(id)
    }

    pub async fn get_status(&self, id: &Uuid) -> Option<PlaybackStatus> {
        let playbacks = self.active_playbacks.lock().await;
        playbacks.get(id).map(|_| PlaybackStatus {
            id: *id,
            playing: true,
            file_name: "Currently playing".to_string(),
        })
    }

    pub async fn list_active_playbacks(&self) -> Vec<Uuid> {
        let playbacks = self.active_playbacks.lock().await;
        playbacks.keys().cloned().collect()
    }

    pub async fn stop_audio(&self, id: &Uuid) -> Result<(), HardwareError> {
        let mut playbacks = self.active_playbacks.lock().await;
        if playbacks.remove(id).is_some() {
            // Kill the specific mpg123 process
            if let Err(e) = Command::new("pkill")
                .arg("-f")
                .arg(format!("mpg123.*{}", id))
                .status()
                .await
            {
                error!("Failed to stop audio playback: {}", e);
            }
            Ok(())
        } else {
            Err(HardwareError::NotFound("Playback not found".to_string()))
        }
    }

    pub async fn stop_all(&self) -> Result<(), HardwareError> {
        let mut playbacks = self.active_playbacks.lock().await;
        playbacks.clear();

        // Kill all mpg123 processes
        if let Err(e) = Command::new("pkill").arg("mpg123").status().await {
            error!("Failed to stop all audio playbacks: {}", e);
            return Err(HardwareError::Other("Failed to stop audio".to_string()));
        }

        Ok(())
    }
}
