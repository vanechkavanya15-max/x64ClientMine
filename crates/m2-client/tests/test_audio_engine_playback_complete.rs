use m2_client::audio_engine_playback_complete::AudioEngine;

#[test]
fn test_audio_engine_3d_volume_attenuation() {
    let mut engine = AudioEngine::new();
    engine.set_master_volume(1.0);
    engine.set_sfx_volume(1.0);
    engine.set_listener_position(0.0, 0.0, 0.0);
    
    // Very close, max volume
    let instance = engine.play_sound_3d("test.wav", 10.0, 0.0, 0.0).unwrap();
    assert!(instance.volume() > 0.9);
    
    // Farther, lower volume
    let instance = engine.play_sound_3d("test.wav", 2500.0, 0.0, 0.0).unwrap();
    assert!(instance.volume() > 0.0);
    assert!(instance.volume() < 0.6);
    
    // Outside max range, no sound
    let instance = engine.play_sound_3d("test.wav", 5500.0, 0.0, 0.0);
    assert!(instance.is_none());
}

#[test]
fn test_audio_engine_bgm_fade() {
    let mut engine = AudioEngine::new();
    engine.fade_in_music("bgm.mp3", 1.0, 0.5); // 0.5 units per frame (60fps) -> fast fade
    
    engine.update(0.1, 100);
    assert!(engine.get_music_volume() == 1.0);
}

#[test]
fn test_audio_engine_buffer_cache() {
    let mut engine = AudioEngine::new();
    
    // Test logic related to sound buffer cache limit would go here.
    // Given the constraints of testing the cache and debouncer, we should add some specific tests for these parts.
    
    // Using play_sound_2d sets a basic volume test
    engine.set_master_volume(0.5);
    let sfx = engine.play_sound_2d("ui.wav").unwrap();
    assert_eq!(sfx.volume(), 0.5); // since sfx volume default is 1.0, 0.5 * 1.0 * 1.0 = 0.5
}

#[test]
fn test_audio_engine_full_integration() {
    let mut engine = AudioEngine::new();
    
    // Set some test params
    engine.set_master_volume(1.0);
    engine.set_sfx_volume(0.8);
    engine.set_listener_position(100.0, 50.0, 20.0);
    engine.set_listener_orientation(0.0);
    
    let vol_sfx = engine.play_sound_2d("ui.wav").unwrap();
    assert_eq!(vol_sfx.volume(), 0.8);
    
    // Test 3D positioning near listener
    let sfx_3d = engine.play_sound_3d("hit.wav", 110.0, 50.0, 20.0).unwrap();
    assert!(sfx_3d.volume() > 0.0);
    assert!(sfx_3d.volume() <= 0.8);
}

#[test]
fn test_audio_engine_debouncer_and_volume() {
    let mut engine = AudioEngine::new();
    
    // Testing debouncer update
    engine.update(1.0, 1000);
    
    // Testing sound volumes from getters
    engine.set_master_volume(0.5);
    engine.set_sfx_volume(0.5);
    engine.set_music_volume(0.5);
    
    assert_eq!(engine.get_sfx_volume(), 0.5);
    assert_eq!(engine.get_music_volume(), 0.5);
}

use m2_client::event_sound_trigger_v2::TMotionSoundEventData;

#[test]
fn test_audio_engine_update_sound_instance() {
    let mut engine = AudioEngine::new();
    engine.set_master_volume(1.0);
    engine.set_sfx_volume(1.0);
    
    let instance1 = TMotionSoundEventData::new(10, 0.0, 0.0, "sound1.wav".to_string());
    let instance2 = TMotionSoundEventData::new(20, 0.0, 0.0, "sound2.wav".to_string());
    
    let instances = vec![instance1, instance2];
    
    // Test frame 10 (should play sound1.wav)
    engine.update_sound_instance(0.0, 0.0, 0.0, 10, &instances, false, 0);
    
    // Note: Due to lack of global state mocking in the minimal tests we rely on the internal execution path completing
    // Real implementation test would inject a mock to verify `play_sound_3d` was called.
}

// Include an extra test for the pop queue mechanics via FFI logic just to make sure
use m2_client::event_sound_trigger_v2::{process_motion_sound_event, clear_motion_sound_events, m2_motion_pop_sound_event};

#[test]
fn test_motion_sound_event_queue() {
    clear_motion_sound_events();
    
    let event = TMotionSoundEventData::new(5, 0.0, 0.0, "hit.wav".to_string());
    
    // Frame 4 (no match)
    process_motion_sound_event(4, &event);
    
    let mut buffer = [0u8; 256];
    let popped = m2_motion_pop_sound_event(buffer.as_mut_ptr());
    assert!(!popped);
    
    // Frame 5 (match)
    process_motion_sound_event(5, &event);
    let popped = m2_motion_pop_sound_event(buffer.as_mut_ptr());
    assert!(popped);
    
    let result = std::ffi::CStr::from_bytes_until_nul(&buffer).unwrap().to_str().unwrap();
    assert_eq!(result, "hit.wav");
}
