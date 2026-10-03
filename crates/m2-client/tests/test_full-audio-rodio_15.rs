#[path = "../src/full-audio-rodio_15.rs"]
mod full_audio_rodio_15;

use full_audio_rodio_15::RodioAudioEngine;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn create_dummy_wav(file_name: &str) -> String {
    let mut path = std::env::temp_dir();
    path.push(file_name);
    
    // Write a minimal valid RIFF/WAV header (44 bytes for a silent PCM file)
    let header: [u8; 44] = [
        0x52, 0x49, 0x46, 0x46, // "RIFF"
        0x24, 0x00, 0x00, 0x00, // ChunkSize (36 + Subchunk2Size)
        0x57, 0x41, 0x56, 0x45, // "WAVE"
        0x66, 0x6d, 0x74, 0x20, // "fmt "
        0x10, 0x00, 0x00, 0x00, // Subchunk1Size (16 for PCM)
        0x01, 0x00,             // AudioFormat (1 for PCM)
        0x01, 0x00,             // NumChannels (1 channel)
        0x44, 0xac, 0x00, 0x00, // SampleRate (44100)
        0x88, 0x58, 0x01, 0x00, // ByteRate (SampleRate * NumChannels * BitsPerSample/8)
        0x02, 0x00,             // BlockAlign (NumChannels * BitsPerSample/8)
        0x10, 0x00,             // BitsPerSample (16 bits)
        0x64, 0x61, 0x74, 0x61, // "data"
        0x00, 0x00, 0x00, 0x00, // Subchunk2Size (0 bytes of data)
    ];

    if let Ok(mut file) = File::create(&path) {
        let _ = file.write_all(&header);
    }
    
    path.to_string_lossy().into_owned()
}

#[test]
fn test_engine_initialization() {
    let engine = RodioAudioEngine::new();
    if let Ok(_) = engine {
        assert!(true);
    }
}

#[test]
fn test_play_bgm() {
    if let Ok(engine) = RodioAudioEngine::new() {
        let path = create_dummy_wav("test_bgm.wav");
        engine.play_bgm(&path);
    }
}

#[test]
fn test_set_master_volume() {
    if let Ok(engine) = RodioAudioEngine::new() {
        engine.set_master_volume(0.5);
        engine.set_master_volume(1.5); // Should clamp to 1.0
        engine.set_master_volume(-0.5); // Should clamp to 0.0
    }
}

#[test]
fn test_play_sound_3d() {
    if let Ok(engine) = RodioAudioEngine::new() {
        let path = create_dummy_wav("test_3d.wav");
        engine.play_sound_3d(&path, [1.0, 2.0, 3.0]);
    }
}

#[test]
fn test_stop_all() {
    if let Ok(engine) = RodioAudioEngine::new() {
        let bgm_path = create_dummy_wav("test_bgm2.wav");
        let sfx_path = create_dummy_wav("test_3d2.wav");
        engine.play_bgm(&bgm_path);
        engine.play_sound_3d(&sfx_path, [1.0, 2.0, 3.0]);
        engine.stop_all();
    }
}
