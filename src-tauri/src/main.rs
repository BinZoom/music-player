// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod audio_service;

use audio_service::{AudioEvent, AudioFile, AudioService};
use rodio::Sink;
use std::sync::Arc;
use tokio::sync::broadcast::Sender;
use tokio::sync::Mutex;

/// Receive events sent by the front end, encapsulate them as [`AudioEvent`] and send them to the channel.
#[tauri::command]
fn handle_event(sender: tauri::State<Sender<AudioEvent>>, event: String) {
    let event: serde_json::Value = serde_json::from_str(&event).unwrap();
    if let Some(action) = event["action"].as_str() {
        match action {
            "play" => event["file_path"]
                .as_str()
                .map(|file_path| sender.send(AudioEvent::Play(file_path.to_owned()))),
            "pause" => Some(sender.send(AudioEvent::Pause)),
            "recovery" => Some(sender.send(AudioEvent::Recovery)),
            "volume" => event["volume"]
                .as_f64()
                .map(|volume| sender.send(AudioEvent::Volume(volume as f32))),
            _ => None, // other actions
        };
    }
}

/// Get audio file information in the target directory.
#[tauri::command]
fn scan_files_in_directory(path: &str) -> Vec<AudioFile> {
    use std::fs;
    let mut counter = 1;
    match fs::read_dir(path) {
        Ok(entries) => entries
            .filter_map(|entry_result| {
                entry_result.ok().and_then(|entry| {
                    let file_name =
                        entry
                            .file_name()
                            .into_string()
                            .ok()
                            .map(|file_name| AudioFile {
                                id: counter,
                                file_name,
                            });

                    if file_name.is_some() {
                        counter += 1;
                    }

                    file_name
                })
            })
            .collect(),
        Err(_) => {
            eprintln!("Failed to read directory '{}'.", path);
            Vec::new()
        }
    }
}

/// Check if there is no source in the sink.
#[tauri::command]
async fn is_sink_empty(sink: tauri::State<'_, Arc<Mutex<Sink>>>) -> Result<bool, String> {
    let sink_clone = Arc::clone(&sink);
    let sink = sink_clone.lock().await;
    let is_empty = sink.empty();
    Ok(is_empty)
}

/// Main method to start the service.
#[tokio::main]
async fn main() {
    let audio_service = AudioService::new();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            handle_event,
            scan_files_in_directory,
            is_sink_empty
        ])
        .manage(audio_service.event_sender) // share
        .manage(audio_service.sink)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fs() {
        let audio_files = scan_files_in_directory("E://music/");
        for audio_file in &audio_files {
            println!("ID: {}, File Name: {}", audio_file.id, audio_file.file_name);
        }
    }
}
