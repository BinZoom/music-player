# MusicPlayer
A simple desktop music player application written in Rust. The front end is based on [Tauri](https://github.com/tauri-apps/tauri) (Vue + Typescript), while the backend utilizes [Tokio](https://github.com/tokio-rs/tokio) & [Rodio](https://github.com/RustAudio/rodio).

### Features:
- Music play and pause.
- Volume adjustment.
- Audio switching.

### How to run
Preparation environment:
- Rust Basic Environment
- Node.js

Modify the audio repository path and place the downloaded audio in it.Search `audioHubPath` in the project for custom modifications.

Finally, install the dependencies and start
```
npm install
npm run tauri dev
```