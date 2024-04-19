// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use audio_service::AudioService;
use serde::Serialize;
use audio_service::AudioEvent;
use tokio::sync::broadcast::Sender;

mod audio_service {
    use rodio::{Decoder, OutputStream, Sink};
    use std::fs::File;
    use std::io::BufReader;
    use std::sync::{Arc, Mutex};
    use tokio::sync::broadcast;
    use tokio::sync::broadcast::Sender;

    #[derive(Debug, Clone)]
    pub enum AudioEvent {
        Play(String),
        Pause,
    }

    pub struct AudioService {
        sink: Arc<Mutex<Sink>>,
        pub event_sender: Sender<AudioEvent>,
    }

    impl AudioService {
        pub fn new() -> Self {
            // Create a tokio broadcast channel to transmit events.
            let (event_sender, mut event_receiver) = broadcast::channel(100);

            let (_stream, handle) = OutputStream::try_default().unwrap();
            // Create a Rodio sink and use Arc and Mutex to share data. If not. The ownership of the sink will be Moved and the sink will not be able to be used in the future.
            let sink = Arc::new(Mutex::new(Sink::try_new(&handle).unwrap()));
            let sink_clone = Arc::clone(&sink);

            // Spawn a task to receive and process events
            tokio::spawn(async move {
                while let Ok(event) = event_receiver.recv().await {
                    match event {
                        AudioEvent::Play(file_path) => {
                            //
                            Self::play_audio(&file_path, &sink_clone).await;
                        }
                        AudioEvent::Pause => {
                            Self::pause_audio(&sink_clone).await;
                        }
                    }
                }
            });

            Self { sink, event_sender }
        }

        async fn play_audio(file_path: &str, sink: &Arc<Mutex<Sink>>) {
            let sink = sink.lock().unwrap();
            // TODO: exception handling
            let file = BufReader::new(File::open(file_path).unwrap());
            let source = Decoder::new(file).unwrap();
            sink.append(source);
        }

        async fn pause_audio(sink: &Arc<Mutex<Sink>>) {
            let sink = sink.lock().unwrap();
            sink.pause();
        }
    }
}


#[tauri::command]
fn handle_event(sender: tauri::State<Sender<AudioEvent>>, event: String) {
    println!("Received event: {}", event);
    let event: serde_json::Value = serde_json::from_str(&event).unwrap();
    if let Some(action) = event["action"].as_str() {
        match action {
            "play" => {
                if let Some(file_path) = event["file_path"].as_str() {
                    sender.send(AudioEvent::Play(file_path.to_owned())).unwrap();
                }
                // 可能还需要处理文件路径为空的情况
            }
            "pause" => {
                sender.send(AudioEvent::Pause).unwrap();
            }
            _ => {
                // 处理其他动作的情况
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
}
