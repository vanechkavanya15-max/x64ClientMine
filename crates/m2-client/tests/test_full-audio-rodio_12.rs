#[path = "../src/animation_sound_event_sync.rs"]
pub mod animation_sound_event_sync;

use m2_client::sound_ffi::{get_audio_state, m2_audio_init};
use animation_sound_event_sync::AnimationSoundSync;
use serial_test::serial;

fn setup_audio() {
    m2_audio_init();
    let mut state = get_audio_state().lock().unwrap();
    state.is_muted = false;
    state.master_volume = 1.0;
    state.last_played_3d = None;
}

const DUMMY_MSA_CONTENT: &str = r#"
ScriptType               MotionData
MotionFileName           "d:/ymir work/pc/warrior/onehand_sword/combo_01.gr2"
MotionDuration           1.333333

Group MotionEventData
{
    MotionEventDataCount     2
    Group Event00
    {
        MotionEventType      6
        StartingTime         0.500000
        SoundFileName        "sound/pc/warrior/sword/swing_1.wav"
    }
    Group Event01
    {
        MotionEventType      6
        StartingTime         1.000000
        SoundFileName        "sound/pc/warrior/sword/swing_2.wav"
    }
}
"#;

#[test]
#[serial]
fn test_sync_no_sound_before_starting_time() {
    setup_audio();
    let mut sync = AnimationSoundSync::new();
    assert!(sync.load_from_msa(DUMMY_MSA_CONTENT).is_ok());
    
    // Update by 0.4 seconds, event 1 is at 0.5s, no sound should play
    sync.update(0.4, 10.0, 20.0, 30.0, 1.0);
    
    let state = get_audio_state().lock().unwrap();
    assert!(state.last_played_3d.is_none());
}

#[test]
#[serial]
fn test_sync_plays_sound_when_crossing_time() {
    setup_audio();
    let mut sync = AnimationSoundSync::new();
    assert!(sync.load_from_msa(DUMMY_MSA_CONTENT).is_ok());
    
    // Cross 0.5s
    sync.update(0.6, 10.0, 20.0, 30.0, 0.8);
    
    let state = get_audio_state().lock().unwrap();
    let last = state.last_played_3d.as_ref().unwrap();
    assert_eq!(last.0, "sound/pc/warrior/sword/swing_1.wav");
    assert_eq!(last.1, 10.0);
    assert_eq!(last.2, 20.0);
    assert_eq!(last.3, 30.0);
    assert_eq!(last.4, 0.8);
}

#[test]
#[serial]
fn test_sync_plays_second_sound_later() {
    setup_audio();
    let mut sync = AnimationSoundSync::new();
    assert!(sync.load_from_msa(DUMMY_MSA_CONTENT).is_ok());
    
    // Update to 0.6s -> triggers swing_1
    sync.update(0.6, 10.0, 20.0, 30.0, 1.0);
    
    {
        let mut state = get_audio_state().lock().unwrap();
        assert_eq!(state.last_played_3d.as_ref().unwrap().0, "sound/pc/warrior/sword/swing_1.wav");
        state.last_played_3d = None; // Reset
    }
    
    // Update to 1.1s (delta 0.5s) -> triggers swing_2
    sync.update(0.5, 0.0, 0.0, 0.0, 1.0);
    
    let state = get_audio_state().lock().unwrap();
    assert_eq!(state.last_played_3d.as_ref().unwrap().0, "sound/pc/warrior/sword/swing_2.wav");
}

#[test]
#[serial]
fn test_sync_does_not_play_twice_if_time_advanced() {
    setup_audio();
    let mut sync = AnimationSoundSync::new();
    assert!(sync.load_from_msa(DUMMY_MSA_CONTENT).is_ok());
    
    // Update to 0.6s -> triggers swing_1
    sync.update(0.6, 10.0, 20.0, 30.0, 1.0);
    {
        let mut state = get_audio_state().lock().unwrap();
        state.last_played_3d = None; // Reset
    }
    
    // Update to 0.8s (delta 0.2s) -> no new sound since 0.5s was already crossed
    sync.update(0.2, 10.0, 20.0, 30.0, 1.0);
    
    let state = get_audio_state().lock().unwrap();
    assert!(state.last_played_3d.is_none());
}

#[test]
#[serial]
fn test_sync_reset_allows_replaying_sounds() {
    setup_audio();
    let mut sync = AnimationSoundSync::new();
    assert!(sync.load_from_msa(DUMMY_MSA_CONTENT).is_ok());
    
    // Update to 0.6s -> triggers swing_1
    sync.update(0.6, 10.0, 20.0, 30.0, 1.0);
    
    sync.reset(); // Reset animation time back to 0.0
    
    {
        let mut state = get_audio_state().lock().unwrap();
        state.last_played_3d = None; // Reset
    }
    
    // Update again to 0.6s -> triggers swing_1 again
    sync.update(0.6, 10.0, 20.0, 30.0, 1.0);
    
    let state = get_audio_state().lock().unwrap();
    assert_eq!(state.last_played_3d.as_ref().unwrap().0, "sound/pc/warrior/sword/swing_1.wav");
}
