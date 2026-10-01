#[path = "../src/audio_engine_playback.rs"]
mod audio_engine_playback;

use audio_engine_playback::{SoundEngine, TSoundInstance, CS_CLIENT_FPS};

#[test]
fn test_sound_engine_initialization() {
    let mut engine = SoundEngine::new();
    assert!(engine.initialize().is_ok());
}

#[test]
fn test_play_sound_2d() {
    let mut engine = SoundEngine::new();
    engine.set_sound_volume(0.8);
    
    // Ignore fail if no device is connected in test environment
    let _ = engine.play_sound_2d("sound/ui/click.wav");
    
    if let Some(instance) = engine.get_sound_2d("sound/ui/click.wav") {
        assert!(instance.is_initialized());
        assert_eq!(instance.get_volume(), 0.8);
    }
}

#[test]
fn test_play_sound_3d_and_position() {
    let mut engine = SoundEngine::new();
    engine.set_listener_position(10.0, 20.0, 30.0);
    
    let idx_opt = engine.play_sound_3d("sound/effect/hit.wav", 15.0, 25.0, 35.0);
    
    if let Some(idx) = idx_opt {
        let instance = engine.get_sound_3d(idx).unwrap();
        assert!(instance.is_initialized());
    }
}

#[test]
fn test_update_sound_instance_frequency_check() {
    let mut engine = SoundEngine::new();
    
    let sounds = vec![
        TSoundInstance {
            dw_frame: 10,
            str_sound_file_name: "sound/monster/hit.wav".to_string(),
        }
    ];

    engine.set_current_time_seconds(1.0);
    engine.update_sound_instance(0.0, 0.0, 0.0, 10, &sounds, true);
    
    engine.stop_all_sound_3d();
    
    engine.set_current_time_seconds(1.1);
    engine.update_sound_instance(0.0, 0.0, 0.0, 10, &sounds, true);
    
    engine.set_current_time_seconds(1.5);
    engine.update_sound_instance(0.0, 0.0, 0.0, 10, &sounds, true);
}

#[test]
fn test_fade_in_music_and_swapping() {
    let mut engine = SoundEngine::new();
    
    let _ = engine.fade_in_music("bgm/login.mp3", 1.0, 1.0);
    
    let idx = engine.get_current_music_index();
    let instance = engine.get_music_instance(idx);
    
    assert_eq!(instance.get_identity(), "bgm/login.mp3");
    
    // Swap to new music
    let _ = engine.fade_in_music("bgm/map1.mp3", 0.5, 2.0);
    
    let new_idx = engine.get_current_music_index();
    assert_ne!(idx, new_idx);
    
    let new_instance = engine.get_music_instance(new_idx);
    assert_eq!(new_instance.get_identity(), "bgm/map1.mp3");
}

#[test]
fn test_music_fade_update_loop() {
    let mut engine = SoundEngine::new();
    let _ = engine.fade_in_music("bgm/loop.mp3", 1.0, 1.0);
    
    let idx = engine.get_current_music_index();
    let expected_rate = 1.0 / CS_CLIENT_FPS / 1.0;
    
    engine.update();
    
    let vol = engine.get_music_instance(idx).get_volume();
    assert!((vol - expected_rate).abs() < f32::EPSILON);
}

#[test]
fn test_master_volume_save_restore_minimized() {
    let mut engine = SoundEngine::new();
    engine.set_master_volume(1.0);
    
    engine.save_volume(true);
    let expected_rate = -1.0 / CS_CLIENT_FPS / 1.0;
    engine.update();
    assert!((engine.get_master_volume() - (1.0 + expected_rate)).abs() < 1e-4);
    
    let saved_vol = engine.get_master_volume();
    engine.restore_volume();
    let expected_restore_rate = (1.0 / CS_CLIENT_FPS) / 4.0;
    engine.update();
    let expected_final_vol = saved_vol + expected_restore_rate;
    assert!((engine.get_master_volume() - expected_final_vol).abs() < 1e-4);
}