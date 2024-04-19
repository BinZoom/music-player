// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use audio_service::AudioService;
use rodio::{Decoder, OutputStream, Sink};
use serde::Serialize;

use audio_service::AudioEvent;
use std::sync::{Arc, Mutex};
use tauri::Manager;
use tokio::sync::broadcast;
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
        pub fn new(sink: Arc<Mutex<Sink>>) -> Self {
            let (event_sender, mut event_receiver) = broadcast::channel(100);
            // 因为 sink 在后续还要用到，在开线程时使用了 move 转移所有权，如果转移了 sink，那么后续就不能再使用 sink 了。所有这里使用了 clone
            let sink_clone = Arc::clone(&sink);
            
            // 创建一个异步任务，用于接收消息
            tokio::spawn(async move {
                let sink = sink_clone; // 使用克隆的 Arc
                while let Ok(event) = event_receiver.recv().await {
                    match event {
                        AudioEvent::Play(file_path) => {
                            Self::play_audio(&file_path, &sink).await;
                        }
                        AudioEvent::Pause => {
                            Self::pause_audio(&sink).await;
                        }
                    }
                }
            });

            Self { sink, event_sender }
        }

        async fn play_audio(file_path: &str, sink: &Arc<Mutex<Sink>>) {
            let file = BufReader::new(File::open(file_path).unwrap());
            let source = Decoder::new(file).unwrap();
            // 获取锁，获取不到则处于阻塞状态
            let mut sink = sink.lock().unwrap();
            sink.append(source);
        }

        async fn pause_audio(sink: &Arc<Mutex<Sink>>) {
            let mut sink = sink.lock().unwrap();
            sink.pause();
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

#[tauri::command]
fn handle_custom_event(state: tauri::State<Arc<Mutex<Sender<AudioEvent>>>>, event: String) {
    let event: serde_json::Value = serde_json::from_str(&event).unwrap();
    let state_clone = Arc::clone(&state);

    let a = event["payload"]["action"].as_str();
    // if let Some(action) = event["payload"]["action"].as_str() {
    //     // 多线程生命周期问题
    //     tokio::spawn(async move {
    //         let sender = state_clone.lock().unwrap();
    //         match action {
    //             "play" => {
    //                 let file_path = event["payload"]["file_path"].as_str().unwrap();
    //                 sender.send(AudioEvent::Play(file_path.to_owned())).unwrap();
    //             }
    //             "pause" => {
    //                 sender.send(AudioEvent::Pause).unwrap();
    //             }
    //             _ => {}
    //         }
    //     });
    // }
}

fn main() {
    let (_stream, handle) = OutputStream::try_default().unwrap();
    // Arc + Mutex 结合的目的是在多线程环境下共享数据，Arc 允许多个所有者共享数据，而 Mutex 确保在任何时候只有一个线程可以访问数据
    let sink = Arc::new(Mutex::new(Sink::try_new(&handle).unwrap()));

    let audio_service = AudioService::new(sink);
    let event_subscriber = audio_service.event_sender;

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            handle_custom_event,
            scan_files_in_directory
        ])
        .manage(event_subscriber)
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
