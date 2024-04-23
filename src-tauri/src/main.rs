// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod audio_service;

use audio_service::{ AudioEvent,AudioService,AudioFile};
use tokio::sync::broadcast::Sender;

/// Receive events sent by the front end, encapsulate them as [`AudioEvent`] and send them to the channel.
#[tauri::command]
fn handle_event(sender: tauri::State<Sender<AudioEvent>>, event: String) {
    let event: serde_json::Value = serde_json::from_str(&event).unwrap();
    if let Some(action) = event["action"].as_str() {
        match action {
            "play" => event["file_path"].as_str().map(|file_path| sender.send(AudioEvent::Play(file_path.to_owned()))),
            "pause" => Some(sender.send(AudioEvent::Pause)),
            "recovery" => Some(sender.send(AudioEvent::Recovery)),
            "volume" => event["volume"].as_f64().map(|volume| sender.send(AudioEvent::Volume(volume as f32))),
            "next" => Some(sender.send(AudioEvent::Next)),
            _ => None, // other actions
        };
    }
}

/// Get audio file information in the target directory.
#[tauri::command]
fn scan_files_in_directory(path: &str) -> Vec<AudioFile> {
    use std::fs;

    match fs::read_dir(path) {
        Ok(entries) => entries
            .filter_map(|entry| {
                entry.ok().and_then(|entry| {
                    entry
                        .file_name()
                        .into_string()
                        .ok()
                        .map(|file_name| AudioFile { file_name })
                })
            })
            .collect(),
        Err(_) => {
            eprintln!("Failed to read directory '{}'.", path);
            Vec::new()
        }
    }
}

/// Main method to start the service.
#[tokio::main]
async fn main() {
    let audio_service = AudioService::new();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            handle_event,
            scan_files_in_directory
        ])
        .manage(audio_service.event_sender) // share
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
