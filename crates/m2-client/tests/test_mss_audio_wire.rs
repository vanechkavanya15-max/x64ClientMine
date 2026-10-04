use std::sync::Mutex;
use std::sync::OnceLock;

// Ensure tests run sequentially when modifying global FFI state.
static TEST_MUTEX: OnceLock<Mutex<()>> = OnceLock::new();

fn get_test_mutex() -> &'static Mutex<()> {
    TEST_MUTEX.get_or_init(|| Mutex::new(()))
}

use m2_client::engine_mss_animation_audio_wire::{MssAudioWire, MssSoundInstance, parse_mss_file};

use m2_client::sound_ffi::get_audio_state;

#[test]
fn test_mss_parser_valid_file() {
    let mss_content = r#"ScriptType        CharacterSoundInformation

SoundDataCount    2
SoundData00       0.533333 "sound/pc/warrior/skill/samyeon_v.wav"
SoundData01       1.200000 "sound/pc/warrior/skill/samyeon_hit.wav"
"#;

    let instances = parse_mss_file(mss_content, 60).unwrap();
    assert_eq!(instances.len(), 2);
    
    // 0.533333 / (1.0 / 60) = 0.533333 * 60 = 31.99998 -> 31
    assert_eq!(instances[0].frame, 31);
    assert_eq!(instances[0].sound_file_name, "sound/pc/warrior/skill/samyeon_v.wav");
    
    // 1.2 * 60 = 72
    assert_eq!(instances[1].frame, 72);
    assert_eq!(instances[1].sound_file_name, "sound/pc/warrior/skill/samyeon_hit.wav");
}

#[test]
fn test_mss_parser_invalid_file() {
    let mss_content = r#"ScriptType        CharacterSoundInformation
    
NoCountHere
"#;
    let res = parse_mss_file(mss_content, 60);
    assert!(res.is_err());
    assert_eq!(res.unwrap_err(), "File format error, SoundDataCount Unable to find.");
}

#[test]
fn test_mss_audio_wire_frequency_check() {
    let _lock = get_test_mutex().lock().unwrap();

    let mut audio_wire = MssAudioWire::new();
    let instances = vec![
        MssSoundInstance {
            frame: 10,
            sound_file_name: "test_sound.wav".to_string(),
        }
    ];

    // FFI mock test, we mock last_played by directly inspecting the state
    // since m2_audio_init will fail in headless Linux without actual ALSA device.
    // Instead we will inject directly into state to bypass the stream thread creation error.
    
    // Clear state & pretend it's initialized so m2_audio_play_sound_3d doesn't abort.
    {
        let mut state = get_audio_state().lock().unwrap();
        state.is_initialized = true;
        state.is_muted = false;
        state.master_volume = 1.0;
        state.last_played_3d = None;
    }

    // Play at t=0.0
    audio_wire.process_sound_events(
        1.0, 2.0, 3.0,
        10,
        &instances,
        true,
        0.0,
    );

    {
        let state = get_audio_state().lock().unwrap();
        assert!(state.last_played_3d.is_some(), "State was not modified successfully");
        let (name, x, y, z, vol) = state.last_played_3d.clone().unwrap();
        assert_eq!(name, "test_sound.wav");
        assert_eq!(x, 1.0);
        assert_eq!(y, 2.0);
        assert_eq!(z, 3.0);
        assert_eq!(vol, 1.0);
    }

    // Clear state
    {
        let mut state = get_audio_state().lock().unwrap();
        state.last_played_3d = None;
    }

    // Play at t=0.1 (should be blocked by 0.3s cooldown)
    audio_wire.process_sound_events(
        1.0, 2.0, 3.0,
        10,
        &instances,
        true,
        0.1,
    );

    {
        let state = get_audio_state().lock().unwrap();
        assert!(state.last_played_3d.is_none(), "Sound should not play due to frequency check");
    }

    // Play at t=0.4 (should pass cooldown)
    audio_wire.process_sound_events(
        1.0, 2.0, 3.0,
        10,
        &instances,
        true,
        0.4,
    );

    {
        let state = get_audio_state().lock().unwrap();
        assert!(state.last_played_3d.is_some(), "Sound should play after cooldown");
    }
}

#[test]
fn test_mss_audio_wire_trigger_specific_frame() {
    let _lock = get_test_mutex().lock().unwrap();
    
    // Initialize mock state
    {
        let mut state = get_audio_state().lock().unwrap();
        state.is_initialized = true;
        state.is_muted = false;
        state.master_volume = 1.0;
        state.last_played_3d = None;
    }

    let mut audio_wire = MssAudioWire::new();
    let instances = vec![
        MssSoundInstance {
            frame: 20,
            sound_file_name: "hit.wav".to_string(),
        }
    ];

    // Clear state
    {
        let mut state = get_audio_state().lock().unwrap();
        state.last_played_3d = None;
    }

    // Frame 19 -> nothing
    audio_wire.process_sound_events(
        5.0, 5.0, 5.0,
        19,
        &instances,
        false,
        0.0,
    );

    {
        let state = get_audio_state().lock().unwrap();
        assert!(state.last_played_3d.is_none());
    }

    // Frame 20 -> triggers
    audio_wire.process_sound_events(
        5.0, 5.0, 5.0,
        20,
        &instances,
        false,
        0.0,
    );

    {
        let state = get_audio_state().lock().unwrap();
        assert!(state.last_played_3d.is_some(), "State was not modified successfully on frame 20");
        let (name, x, y, z, vol) = state.last_played_3d.clone().unwrap();
        assert_eq!(name, "hit.wav");
        assert_eq!(x, 5.0);
        assert_eq!(y, 5.0);
        assert_eq!(z, 5.0);
        assert_eq!(vol, 1.0);
    }
}
