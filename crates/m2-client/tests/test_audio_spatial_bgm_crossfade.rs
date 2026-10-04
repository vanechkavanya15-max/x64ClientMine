use m2_client::audio_spatial_bgm_crossfade::{SpatialSoundEngine, Vector3};

#[test]
fn test_fade_in_and_crossfade() {
    let mut engine = SpatialSoundEngine::new();
    
    // Start track 1
    engine.fade_in_music("track1.mp3", 1.0, 2.0);
    assert_eq!(engine.current_music_index, 1); // Swapped from 0 to 1
    assert_eq!(engine.music_tracks[1].identity, "track1.mp3");
    
    // Update halfway
    engine.update(1.0);
    assert_eq!(engine.music_tracks[1].volume, 0.5);
    
    // Update to end
    engine.update(1.0);
    assert_eq!(engine.music_tracks[1].volume, 1.0);
    assert_eq!(engine.music_tracks[1].fade_rate_per_sec, 0.0);
    
    // Start track 2, which should crossfade
    engine.fade_in_music("track2.mp3", 1.0, 2.0);
    assert_eq!(engine.current_music_index, 0); // Swapped back to 0
    assert_eq!(engine.music_tracks[0].identity, "track2.mp3");
    
    // Check old track is fading out
    assert_eq!(engine.music_tracks[1].fade_target_volume, 0.0);
    assert!(engine.music_tracks[1].fade_rate_per_sec < 0.0);
    
    // Update halfway
    engine.update(1.0);
    assert_eq!(engine.music_tracks[0].volume, 0.5);
    assert_eq!(engine.music_tracks[1].volume, 0.5);
    
    // Update to end
    engine.update(1.0);
    assert_eq!(engine.music_tracks[0].volume, 1.0);
    assert_eq!(engine.music_tracks[1].volume, 0.0);
    assert_eq!(engine.music_tracks[1].is_playing, false);
}

#[test]
fn test_listener_orientation() {
    let mut engine = SpatialSoundEngine::new();
    
    // C++ requires negating Z for forward, and Y for up
    engine.set_listener_orientation(1.0, 2.0, 3.0, 4.0, 5.0, 6.0);
    
    assert_eq!(engine.listener_forward.x, 1.0);
    assert_eq!(engine.listener_forward.y, 2.0);
    assert_eq!(engine.listener_forward.z, -3.0); // Negated Z
    
    assert_eq!(engine.listener_up.x, 4.0);
    assert_eq!(engine.listener_up.y, -5.0); // Negated Y
    assert_eq!(engine.listener_up.z, 6.0);
}

#[test]
fn test_play_sound_3d() {
    let mut engine = SpatialSoundEngine::new();
    engine.set_listener_position(10.0, 20.0, 30.0);
    engine.set_sound_volume(0.8);
    
    let instance = engine.play_sound_3d("explosion.wav", 15.0, 25.0, 35.0).unwrap();
    
    // Position should be relative to listener
    assert_eq!(instance.position.x, 5.0);
    assert_eq!(instance.position.y, 5.0);
    assert_eq!(instance.position.z, 5.0);
    
    assert_eq!(instance.identity, "explosion.wav");
    assert_eq!(instance.volume, 0.8);
    assert_eq!(instance.is_playing, true);
    
    engine.stop_all_sound_3d();
    assert_eq!(engine.sounds_3d[0].is_playing, false);
}

#[test]
fn test_fade_out_all_music() {
    let mut engine = SpatialSoundEngine::new();
    
    // Setup two playing tracks
    engine.music_tracks[0].identity = "trackA.mp3".to_string();
    engine.music_tracks[0].volume = 1.0;
    engine.music_tracks[0].is_playing = true;
    
    engine.music_tracks[1].identity = "trackB.mp3".to_string();
    engine.music_tracks[1].volume = 0.5;
    engine.music_tracks[1].is_playing = true;
    
    engine.fade_out_all_music();
    
    assert!(engine.music_tracks[0].fade_rate_per_sec < 0.0);
    assert!(engine.music_tracks[1].fade_rate_per_sec < 0.0);
    
    // Update enough time to finish fade (2.0s duration used in fade_out_all_music)
    engine.update(2.1);
    
    assert_eq!(engine.music_tracks[0].volume, 0.0);
    assert_eq!(engine.music_tracks[0].is_playing, false);
    
    assert_eq!(engine.music_tracks[1].volume, 0.0);
    assert_eq!(engine.music_tracks[1].is_playing, false);
}
