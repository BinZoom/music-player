// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use audio_service::AudioEvent;
use audio_service::AudioService;
use serde::Serialize;
use tokio::sync::broadcast::Sender;

mod audio_service {
    use rodio::{Decoder, OutputStream, Sink};
    use std::fs::File;
    use std::io::BufReader;
    use std::sync::Arc;
    use tokio::sync::{broadcast, Mutex};
    use tokio::sync::broadcast::Sender;

    #[derive(Debug, Clone)]
    pub enum AudioEvent {
        Play(String),
        Recovery,
        Pause,
    }

    pub struct AudioService {
        pub event_sender: Sender<AudioEvent>,
        _stream: OutputStream, // sink need the stream, ensuring that their lifecycles are the same
        sink: Arc<Mutex<Sink>>,
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
                    println!("Received event");
                    match event {
                        AudioEvent::Play(file_path) => {
                            println!("Play {}", file_path);
                            let sink = sink_clone.lock().await;
                            let file = BufReader::new(File::open(file_path).unwrap());
                            let source = Decoder::new(file).unwrap();
                            sink.append(source);
                            // sink.sleep_until_end();
                        }
                        AudioEvent::Recovery => {
                            println!("Recovery");
                            let sink = sink_clone.lock().await;
                            sink.play();
                        }
                        AudioEvent::Pause => {
                            println!("Pause");
                            let sink = sink_clone.lock().await;
                            sink.pause();
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
}

#[tauri::command]
fn handle_event(sender: tauri::State<Sender<AudioEvent>>, event: String) {
    let event: serde_json::Value = serde_json::from_str(&event).unwrap();
    if let Some(action) = event["action"].as_str() {
        match action {
            "play" => {
                if let Some(file_path) = event["file_path"].as_str() {
                    sender.send(AudioEvent::Play(file_path.to_owned())).unwrap();
                    println!("send event: {}", file_path);
                }
            }
            "pause" => {
                sender.send(AudioEvent::Pause).unwrap();
            }
            "recovery" => {
                sender.send(AudioEvent::Recovery).unwrap();
            }
            _ => {
                // other actions
            }
        }
    }
}

#[derive(Serialize, Debug)]
pub struct MusicFile {
    pub file_name: String,
}

// 读取指定目录下的所有文件
#[tauri::command]
fn scan_files_in_directory(path: &str) -> Vec<MusicFile> {
    use std::fs;

    match fs::read_dir(path) {
        Ok(entries) => entries
            .filter_map(|entry| {
                entry.ok().and_then(|entry| {
                    entry
                        .file_name()
                        .into_string()
                        .ok()
                        .map(|file_name| MusicFile { file_name })
                })
            })
            .collect(),
        Err(_) => {
            eprintln!("Failed to read directory '{}'.", path);
            Vec::new()
        }
    }
}
#[tokio::main]
async fn main() {
    let audio_service = AudioService::new();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            handle_event,
            scan_files_in_directory
        ])
        .manage(audio_service.event_sender)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

mod unit_tests {
    #[test]
    fn test_scan_files_in_directory() {
        use crate::scan_files_in_directory;
        let m1 = scan_files_in_directory("E://music/");
        println!("{:?}", m1);
    }

    #[test]
    fn test_rodio() {
        use rodio::{Decoder, OutputStream, Sink};
        use std::fs::File;
        use std::io::BufReader;

        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        // Add a dummy source of the sake of the example.
        let file = BufReader::new(File::open("E://music/任然-飞鸟和蝉.flac").unwrap());
        let source = Decoder::new(file).unwrap();
        sink.append(source);

        // The sound plays in a separate thread. This call will block the current thread until the sink
        // has finished playing all its queued sounds.
        sink.sleep_until_end();
    }
}
