# MusicPlayer
A simple desktop music player application written in Rust. The front end is based on [Tauri](https://github.com/tauri-apps/tauri) (Vue + Typescript), while the backend utilizes [Tokio](https://github.com/tokio-rs/tokio) & [Rodio](https://github.com/RustAudio/rodio).

### Features:
- Music play and pause.
- Volume adjustment.
- Audio switching.

### How to run
1. Preparation environment
- Rust（ C++ Env）
- Node.js
2. Modify the audio repository path and place the pre downloaded audio in it.Search 'musicHubPath' in the project for custom modifications.
3. Start project
```
npm install
npm run tauri dev
```