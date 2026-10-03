#[path = "../src/rodio_bgm_streamer.rs"]
mod rodio_bgm_streamer;

use rodio_bgm_streamer::BgmStreamer;
use std::fs::File;
use std::io::Write;
use tempfile::NamedTempFile;

// Utility function to create a dummy but valid MP3-like file
// For rodio tests, we might need a real minimal MP3 if it's strictly checking headers
// For now, we test invalid file handling, and for valid we'll just check if it fails appropriately if not a valid MP3
fn create_dummy_file() -> NamedTempFile {
    let mut file = NamedTempFile::new().unwrap();
    file.write_all(b"not an mp3").unwrap();
    file
}

#[test]
fn test_bgm_streamer_initialization() {
    let streamer = BgmStreamer::new();
    // Some CI environments might not have audio output devices, 
    // rodio's try_default() might fail if no audio device is present.
    // In a real environment with audio devices this should succeed.
    // Assuming tests run in an environment with audio devices (or dummy audio).
    if let Ok(st) = streamer {
        assert_eq!(st.current_volume(), 1.0);
    }
}

#[test]
fn test_play_from_invalid_file() {
    let mut streamer = match BgmStreamer::new() {
        Ok(s) => s,
        Err(_) => return, // Skip test if no audio device
    };
    
    let dummy_file = create_dummy_file();
    let result = streamer.play_from_file(dummy_file.path());
    
    // Should fail because it's not a valid MP3
    assert!(result.is_err());
}

#[test]
fn test_play_from_invalid_memory() {
    let mut streamer = match BgmStreamer::new() {
        Ok(s) => s,
        Err(_) => return, // Skip test if no audio device
    };
    
    let result = streamer.play_from_memory(vec![0, 1, 2, 3]);
    
    // Should fail because it's not a valid MP3 buffer
    assert!(result.is_err());
}

#[test]
fn test_volume_control() {
    let mut streamer = match BgmStreamer::new() {
        Ok(s) => s,
        Err(_) => return, // Skip test if no audio device
    };
    
    streamer.set_volume(0.5);
    assert_eq!(streamer.current_volume(), 0.5);
    
    // Test clamping
    streamer.set_volume(1.5);
    assert_eq!(streamer.current_volume(), 1.0);
    
    streamer.set_volume(-0.5);
    assert_eq!(streamer.current_volume(), 0.0);
}

#[test]
fn test_playback_controls_state() {
    let mut streamer = match BgmStreamer::new() {
        Ok(s) => s,
        Err(_) => return, // Skip test if no audio device
    };
    
    // Initially not playing
    assert!(!streamer.is_playing());
    
    // We can't easily test is_playing == true without a valid MP3, 
    // but we can test that calling pause/resume/stop doesn't panic
    streamer.pause();
    streamer.resume();
    streamer.stop();
    
    assert!(!streamer.is_playing());
}
