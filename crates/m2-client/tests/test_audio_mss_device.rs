use m2_client::audio_mss_device::*;
use std::sync::{Mutex, OnceLock};

// Global lock to ensure tests run sequentially and don't interfere with global state
static TEST_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

fn lock_tests() -> std::sync::MutexGuard<'static, ()> {
    TEST_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap()
}

#[test]
fn test_init_mss_sound_device_fallback() {
    let _guard = lock_tests();
    reset_audio_state_for_test();
    
    // Test initialization - on linux it should fail with silent fallback, 
    // on windows it might fail if mss32.dll is not present.
    let result = init_mss_sound_device();
    
    // In both failure cases, it should return an Err with a string
    // This allows silent fallback in the game engine without crashing.
    if result.is_err() {
        let err_msg = result.unwrap_err();
        assert!(err_msg.contains("mss32.dll") || err_msg.contains("card not present"));
    }
}

#[test]
fn test_play_sound_effect_without_init_is_silent() {
    let _guard = lock_tests();
    reset_audio_state_for_test();
    
    // When not initialized (device.is_initialized = false)
    // play_sound_effect should silently return without inserting to playing_sounds
    play_sound_effect(100, 1.0);
    
    assert_eq!(is_sound_playing_for_test(100), None);
}

#[test]
fn test_play_sound_effect_success_with_clamping() {
    let _guard = lock_tests();
    reset_audio_state_for_test();
    mock_init_success_for_test();
    
    // Test playing a sound with volume > 1.0
    play_sound_effect(200, 1.5);
    
    // Should be clamped to 1.0
    assert_eq!(is_sound_playing_for_test(200), Some(1.0));
    
    // Test playing a sound with volume < 0.0
    play_sound_effect(201, -0.5);
    
    // Should be clamped to 0.0
    assert_eq!(is_sound_playing_for_test(201), Some(0.0));
    
    // Test normal volume
    play_sound_effect(202, 0.75);
    
    assert_eq!(is_sound_playing_for_test(202), Some(0.75));
}

#[test]
fn test_stop_sound_effect_without_init_is_silent() {
    let _guard = lock_tests();
    reset_audio_state_for_test();
    
    // Even if we mock a playing sound by force, stop_sound_effect should do nothing
    // if device is not initialized.
    mock_init_success_for_test();
    play_sound_effect(300, 1.0);
    
    // Now simulate losing initialization
    reset_audio_state_for_test();
    
    // Need to re-inject the playing sound for the test condition
    // But since reset clears everything, this test just verifies it doesn't panic.
    stop_sound_effect(300);
}

#[test]
fn test_stop_sound_effect_success() {
    let _guard = lock_tests();
    reset_audio_state_for_test();
    mock_init_success_for_test();
    
    play_sound_effect(400, 1.0);
    assert_eq!(is_sound_playing_for_test(400), Some(1.0));
    
    stop_sound_effect(400);
    assert_eq!(is_sound_playing_for_test(400), None);
    
    // Stopping a non-existent sound should not panic
    stop_sound_effect(999);
}

#[test]
fn test_resource_struct_instantiation() {
    let _guard = lock_tests();
    reset_audio_state_for_test();
    
    let resource = SoundEffectResource {
        vnum: 123,
        file_path: "sound/effect/hit.wav".to_string(),
        sound_id: 500,
    };
    
    add_resource_for_test(resource.clone());
    
    assert_eq!(resource.vnum, 123);
    assert_eq!(resource.file_path, "sound/effect/hit.wav");
    assert_eq!(resource.sound_id, 500);
}
