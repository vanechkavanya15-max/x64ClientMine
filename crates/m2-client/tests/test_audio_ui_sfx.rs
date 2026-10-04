use rodio::OutputStream;
use std::fs::File;
use std::io::Write;
use tempfile::tempdir;

use m2_client::audio_ui_sound_effects_dispatcher;

use audio_ui_sound_effects_dispatcher::{AudioUiSoundEffectsDispatcher, DispatcherError, UiSoundEvent};

#[test]
fn test_default_registry_mapping() {
    let dispatcher = AudioUiSoundEffectsDispatcher::new_for_test();
    
    assert_eq!(dispatcher.get_sound_path(UiSoundEvent::UI_CLICK_BUTTON).unwrap(), "sound/ui/click.wav");
    assert_eq!(dispatcher.get_sound_path(UiSoundEvent::UI_OPEN_WINDOW).unwrap(), "sound/ui/open_window.wav");
    assert_eq!(dispatcher.get_sound_path(UiSoundEvent::UI_CLOSE_WINDOW).unwrap(), "sound/ui/close_window.wav");
    assert_eq!(dispatcher.get_sound_path(UiSoundEvent::UI_DROP_ITEM).unwrap(), "sound/ui/drop.wav");
    assert_eq!(dispatcher.get_sound_path(UiSoundEvent::UI_PICK_ITEM).unwrap(), "sound/ui/pick.wav");
    assert_eq!(dispatcher.get_sound_path(UiSoundEvent::UI_ERROR_NOTIFY).unwrap(), "sound/ui/error.wav");
}

#[test]
fn test_no_stream_handle_error() {
    let dispatcher = AudioUiSoundEffectsDispatcher::new_for_test();
    let result = dispatcher.play_sound(UiSoundEvent::UI_CLICK_BUTTON);
    
    assert_eq!(result.err().unwrap(), DispatcherError::NoStreamHandle);
}

#[test]
fn test_file_not_found_and_custom_registration() {
    // Generate valid stream handle, but we might be headless, so handle failures gracefully
    // Instead of failing the test if audio device is unavailable, we can still test the logic
    let (_stream, stream_handle) = match OutputStream::try_default() {
        Ok((s, h)) => (s, h),
        Err(_) => return, // Skip test if no audio device is available on CI
    };

    let mut dispatcher = AudioUiSoundEffectsDispatcher::new(stream_handle);
    
    // Register custom path that does not exist
    let missing_path = "non_existent_file_123.wav";
    dispatcher.register_sound_event(UiSoundEvent::UI_ERROR_NOTIFY, missing_path);
    
    assert_eq!(dispatcher.get_sound_path(UiSoundEvent::UI_ERROR_NOTIFY).unwrap(), missing_path);
    
    let result = dispatcher.play_sound(UiSoundEvent::UI_ERROR_NOTIFY);
    assert_eq!(result.err().unwrap(), DispatcherError::FileNotFound);
}

#[test]
fn test_successful_initialization_and_playback() {
    let (_stream, stream_handle) = match OutputStream::try_default() {
        Ok((s, h)) => (s, h),
        Err(_) => return, // Skip test if no audio device is available on CI
    };

    let mut dispatcher = AudioUiSoundEffectsDispatcher::new(stream_handle);
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("dummy.wav");
    
    // Create a minimal valid WAV file
    // 44-byte WAV header + 0 data bytes
    let wav_header: [u8; 44] = [
        0x52, 0x49, 0x46, 0x46, // "RIFF"
        0x24, 0x00, 0x00, 0x00, // Chunk size (36 + data size 0)
        0x57, 0x41, 0x56, 0x45, // "WAVE"
        0x66, 0x6D, 0x74, 0x20, // "fmt "
        0x10, 0x00, 0x00, 0x00, // Subchunk1Size (16 for PCM)
        0x01, 0x00,             // AudioFormat (1 for PCM)
        0x01, 0x00,             // NumChannels (1)
        0x44, 0xAC, 0x00, 0x00, // SampleRate (44100)
        0x88, 0x58, 0x01, 0x00, // ByteRate (44100 * 1 * 2)
        0x02, 0x00,             // BlockAlign (1 * 2)
        0x10, 0x00,             // BitsPerSample (16)
        0x64, 0x61, 0x74, 0x61, // "data"
        0x00, 0x00, 0x00, 0x00, // Subchunk2Size (0)
    ];

    let mut file = File::create(&file_path).unwrap();
    file.write_all(&wav_header).unwrap();
    
    let path_str = file_path.to_str().unwrap();
    dispatcher.register_sound_event(UiSoundEvent::UI_CLICK_BUTTON, path_str);
    
    let sink_result = dispatcher.play_sound(UiSoundEvent::UI_CLICK_BUTTON);
    assert!(sink_result.is_ok(), "Sink should be created successfully");
    
    let sink = sink_result.unwrap();
    // Verify volume is exactly 1.0
    assert_eq!(sink.volume(), 1.0);
}
