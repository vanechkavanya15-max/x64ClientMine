#[path = "../src/py_bridge_audio_full.rs"]
mod py_bridge_audio_full;

use m2_client::sound_ffi;
use std::ffi::CString;

#[test]
fn test_py_bridge_audio_full_bgm() {
    sound_ffi::m2_audio_init();

    let bgm_file = CString::new("test_bgm.mp3").unwrap();
    
    let res = py_bridge_audio_full::m2_py_snd_play_bgm(bgm_file.as_ptr());
    assert!(res, "Should successfully play BGM");

    let state = sound_ffi::get_audio_state().lock().unwrap();
    assert_eq!(state.current_bgm.as_deref(), Some("test_bgm.mp3"), "Current BGM should match");
    drop(state);

    let res = py_bridge_audio_full::m2_py_snd_stop_bgm();
    assert!(res, "Should successfully stop BGM");

    let state = sound_ffi::get_audio_state().lock().unwrap();
    assert_eq!(state.current_bgm, None, "Current BGM should be none");
}

#[test]
fn test_py_bridge_audio_full_set_volumes() {
    sound_ffi::m2_audio_init();

    py_bridge_audio_full::m2_py_snd_set_bgm_volume(0.5);
    
    let state = sound_ffi::get_audio_state().lock().unwrap();
    assert_eq!(state.category_volumes[1], 0.5, "BGM volume should be set");
    drop(state);

    py_bridge_audio_full::m2_py_snd_set_sound_volume(0.8);
    
    let state = sound_ffi::get_audio_state().lock().unwrap();
    assert_eq!(state.category_volumes[0], 0.8, "Sound volume should be set");
    drop(state);

    py_bridge_audio_full::m2_py_snd_set_master_volume(0.7);

    let state = sound_ffi::get_audio_state().lock().unwrap();
    assert_eq!(state.master_volume, 0.7, "Master volume should be set");
}

#[test]
fn test_py_bridge_audio_full_sound() {
    sound_ffi::m2_audio_init();

    let sfx_file = CString::new("test_sfx.wav").unwrap();
    
    let res = py_bridge_audio_full::m2_py_snd_play_sound(sfx_file.as_ptr());
    assert!(res, "Should successfully play 2D sound");

    let res = py_bridge_audio_full::m2_py_snd_play_sound_at_position(sfx_file.as_ptr(), 10.0, 20.0, 30.0);
    assert!(res, "Should successfully play 3D sound");

    let res = py_bridge_audio_full::m2_py_snd_stop_all_sound();
    assert!(res, "Should successfully stop all sounds");
}

#[test]
fn test_py_bridge_audio_full_fade_music() {
    sound_ffi::m2_audio_init();

    let bgm_file = CString::new("test_fade_in.mp3").unwrap();
    let res = py_bridge_audio_full::m2_py_snd_fade_in_music(bgm_file.as_ptr());
    assert!(res, "Should fade in music");

    let state = sound_ffi::get_audio_state().lock().unwrap();
    assert_eq!(state.current_bgm.as_deref(), Some("test_fade_in.mp3"));
    drop(state);

    let res = py_bridge_audio_full::m2_py_snd_fade_out_music(bgm_file.as_ptr());
    assert!(res, "Should fade out music");

    let state = sound_ffi::get_audio_state().lock().unwrap();
    assert_eq!(state.current_bgm, None);
    drop(state);

    let res = py_bridge_audio_full::m2_py_snd_fade_out_all_music();
    assert!(res, "Should fade out all music");

    let limit_vol = 0.2;
    let res = py_bridge_audio_full::m2_py_snd_fade_limit_out_music(bgm_file.as_ptr(), limit_vol);
    assert!(res, "Should fade limit out music");
    let state = sound_ffi::get_audio_state().lock().unwrap();
    assert_eq!(state.category_volumes[1], limit_vol);
    assert_eq!(state.current_bgm, None);
}

#[test]
fn test_py_bridge_audio_full_null_ptrs() {
    sound_ffi::m2_audio_init();

    let res = py_bridge_audio_full::m2_py_snd_play_bgm(std::ptr::null());
    assert!(!res, "Should handle null pointer gracefully");

    let res = py_bridge_audio_full::m2_py_snd_play_sound(std::ptr::null());
    assert!(!res, "Should handle null pointer gracefully");

    let res = py_bridge_audio_full::m2_py_snd_play_sound_at_position(std::ptr::null(), 0.0, 0.0, 0.0);
    assert!(!res, "Should handle null pointer gracefully");
}
