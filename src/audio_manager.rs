use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

pub struct AudioManager {
    // Keep the OutputStream and OutputStreamHandle alive
    _output_stream: OutputStream,
    output_stream_handle: OutputStreamHandle,
    // Store sinks to keep them alive
    sinks: Mutex<HashMap<Uuid, Sink>>,
}

impl AudioManager {
    pub fn new() -> Self {
        let (output_stream, output_stream_handle) =
            OutputStream::try_default().expect("Failed to get default output device");
        AudioManager {
            _output_stream: output_stream,
            output_stream_handle,
            sinks: Mutex::new(HashMap::new()),
        }
    }

    pub fn play_audio(&self, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Open the audio file
        let file = File::open(file_path)?;
        let source = Decoder::new(BufReader::new(file))?;

        // Create a new sink
        let sink = Sink::try_new(&self.output_stream_handle)?;

        // Add the audio source to the sink
        sink.append(source);

        // Generate a unique ID for this sink
        let id = Uuid::new_v4();

        // Store the sink to keep it alive
        {
            let mut sinks = self.sinks.lock().unwrap();
            sinks.insert(id, sink);
        }

        // Optionally, you can clean up finished sinks
        self.cleanup_sinks();

        Ok(())
    }

    fn cleanup_sinks(&self) {
        let mut sinks = self.sinks.lock().unwrap();
        sinks.retain(|_, sink| !sink.empty());
    }
}
