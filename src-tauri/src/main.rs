// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// 以指令的形式暴露给前端应用
#[tauri::command]
fn play() {
    use rodio::source::{SineWave, Source};
    use rodio::{Decoder, OutputStream, Sink};
    use std::fs::File;
    use std::io::BufReader;

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let file = BufReader::new(File::open("examples/吕口口 - 希望你被这个世界爱着.mp3").unwrap());
    let source = Decoder::new(file).unwrap();

    sink.append(source);
    sink.set_volume(0.5);

    // The sound plays in a separate audio thread,
    // so we need to keep the main thread alive while it's playing.
    sink.sleep_until_end();
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![play])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
