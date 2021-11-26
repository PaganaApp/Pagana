# Pagana MVP Design Document

Pagana is a (incomplete) project which is aimed to be a FREE and open-source music player with DAW-like functionality for Linux, Mac and Windows. Its goals are to be an easy and intuitive real-time audio mixer for any type of user while not requiring advanced audio knowledge.

*This project is still in early stages of developpment and is not ready for any kind of production use or testing.*

# Objective

Building any kind of audio software is no easy task. There are multiple moving parts during the development that has to be handled correctl such as memory management, audio input/output and stability.

Ideally, the project would be small, fast, compatible and stable with every platform.

For these reasons, we believe that Rust (along with Tauri) are one of the best options available at the moment for the development of this project. Here are some of the features that come along with them:
- No garbage collection but efficient memory handling (audio-safe)
- Cross-platformm built-in
- Efficient error handling and code stability
- Really helpful compiler and package manager
- Blazing fast backend and web-like view

# Scope for MVP (Minimum viable product)

While having a lot of features that might compete with other product that already exist, we have to focus on certain capabilites that are essential and limit the scope to achieve minimum viable product (MVP) release.

*This MVP document is subject to change*

### Goals

- Crosplatform (Linux, Mac and Windows)
- Multi-track audio reproduction. These tracks can be added, removed and changed in real-time.
- Load AAC, ALAC, FLAC, MP3, MP4, OGG, Vorbis, and WAV files.
- Record session and export to WAV file.
- Effects on tracks (including in the MVP: pitch shift, gain, distortion, reverb and delay).
- Track grouping
- Robust UI
  - Edit mode
    - Left panel with tracks
    - Right panel with selected track information
    - Bottom right subpanel with track secondary information (effects and other mixing options)
  - Play mode
    - Top-most view with tracks
    - Bottom panel with basic track mixing (mute, solo, volume slider, pan knob, track renaming, color changing, track routing).
- Settings
  - Theming (MVP: Light and dark themes)
  - Audio settings (default output device and sample rate)
  - Sound files source
  - Modules storage route
  - User configuration source

### Non-Goals

To keep the scope of the project limited, we will define some non-goals achievements that will come after the official release of Pagana.

- Export session to MP3, OGG, Vorbis, AAC, etc.
- Audio stretching
- Track or effect presets
- Sequenced automation

## Backend-Design (MVP)

to-do

## Frontend-Design (MVP)

to-do

# Main Libraries Used

* [**Tauri**](https://tauri-apps/tauri): Framework for interaction between frontend ([SvelteJS](https://github.com/sveltejs/svelte)) and backend ([Rust](https://github.com/rust-lang/rust)).
* [**Symphonia**](https://github.com/pdeljanov/Symphonia): Rust Audio decodification for AAC, ALAC, FLAC, MP3, MP4, OGG, Vorbis, and WAV files.
* [**cpal**](https://github.com/RustyDAW/cpal): Low-level library for audio input and output for Rust.

*Other small or less-important libraries can be found on the `package.json` or `src-tauri/Cargo.toml` files for the frontend and backend respectively.*
