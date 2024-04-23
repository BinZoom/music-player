/*! Audio service module */

use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;
use tokio::sync::broadcast::Sender;
use tokio::sync::{broadcast, Mutex};
use serde::Serialize;

/// Audio processing services. The main job is to receive [`AudioEvent`] and process them
pub struct AudioService {
    pub event_sender: Sender<AudioEvent>,
    _stream: OutputStream, // sink need the stream, ensuring that their lifecycles are the same
    sink: Arc<Mutex<Sink>>,
}

/// Audio file information.
#[derive(Serialize, Debug)]
pub struct AudioFile {
    pub file_name: String,
}

impl AudioService {
    pub fn new() -> Self {
        // Create a tokio broadcast channel to transmit events.
        let (event_sender, mut event_receiver) = broadcast::channel(100);
        // Create a Rodio sink and use Arc and Mutex to share data. If not. The ownership of the sink will be Moved and the sink will not be able to be used in the future.
        let (_stream, handle) = OutputStream::try_default().unwrap();
        let sink = Arc::new(Mutex::new(Sink::try_new(&handle).unwrap()));
        let sink_clone = Arc::clone(&sink);

        tokio::spawn(async move {
            while let Ok(event) = event_receiver.recv().await {
                let sink = sink_clone.lock().await;
                match event {
                    AudioEvent::Play(file_path) => {
                        let file = BufReader::new(File::open(file_path).unwrap());
                        let source = Decoder::new(file).unwrap();
                        sink.append(source);
                    }
                    AudioEvent::Recovery => {
                        sink.play();
                    }
                    AudioEvent::Pause => {
                        sink.pause();
                    }
                    AudioEvent::Volume(volume) => {
                        sink.set_volume(volume / 50.0);
                    }
                    AudioEvent::Next => {
                        sink.skip_one();
                    }
                }
            }
        });

        Self {
            event_sender,
            _stream,
            sink,
        }
    }
}

#[derive(Debug, Clone)]
pub enum AudioEvent {
    Play(String),
    Recovery,
    Pause,
    Volume(f32),
    Next,
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_rodio() {
        use rodio::{Decoder, OutputStream, Sink};
        use std::fs::File;
        use std::io::BufReader;

        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        // Add a dummy source of the sake of the example.
        let file = BufReader::new(File::open("E://music/music1.flac").unwrap());
        let source = Decoder::new(file).unwrap();
        sink.append(source);

        // The sound plays in a separate thread. This call will block the current thread until the sink
        // has finished playing all its queued sounds.
        sink.sleep_until_end();
    }
}