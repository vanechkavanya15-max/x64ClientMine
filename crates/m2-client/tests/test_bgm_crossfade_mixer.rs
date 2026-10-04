use m2_client::audio_bgm_crossfade_mixer::BgmCrossfadeMixer;

#[test]
fn test_crossfade_math_fade_in() {
    let mut mixer = BgmCrossfadeMixer::new(2.0); // 2 second fade duration
    mixer.play_music("bgm/test1.mp3");

    // After 1 second, volume should be 0.5
    mixer.update(1.0);
    assert_eq!(mixer.current_track.as_ref().unwrap().volume, 0.5);

    // After another 1 second, volume should be 1.0
    mixer.update(1.0);
    assert_eq!(mixer.current_track.as_ref().unwrap().volume, 1.0);

    // Beyond 2 seconds, volume should remain capped at 1.0
    mixer.update(1.0);
    assert_eq!(mixer.current_track.as_ref().unwrap().volume, 1.0);
}

#[test]
fn test_crossfade_math_fade_out() {
    let mut mixer = BgmCrossfadeMixer::new(2.0);
    mixer.play_music("bgm/test1.mp3");
    
    // Fully fade in
    mixer.update(2.0);
    assert_eq!(mixer.current_track.as_ref().unwrap().volume, 1.0);

    // Stop music
    mixer.stop_music();

    // current track is None, previous track is test1.mp3 with volume 1.0
    assert!(mixer.current_track.is_none());
    assert_eq!(mixer.previous_track.as_ref().unwrap().volume, 1.0);

    // After 1 second, previous track volume should be 0.5
    mixer.update(1.0);
    assert_eq!(mixer.previous_track.as_ref().unwrap().volume, 0.5);

    // After another 1 second, previous track volume should be 0.0 and it should be removed
    mixer.update(1.0);
    assert!(mixer.previous_track.is_none());
}

#[test]
fn test_play_new_track_crossfade() {
    let mut mixer = BgmCrossfadeMixer::new(2.0);
    mixer.play_music("bgm/test1.mp3");
    
    // Partially fade in first track
    mixer.update(1.0);
    assert_eq!(mixer.current_track.as_ref().unwrap().volume, 0.5);

    // Play second track
    mixer.play_music("bgm/test2.mp3");

    // current track is test2.mp3 with 0.0 volume
    // previous track is test1.mp3 with 0.5 volume
    assert_eq!(mixer.current_track.as_ref().unwrap().filepath, "bgm/test2.mp3");
    assert_eq!(mixer.current_track.as_ref().unwrap().volume, 0.0);

    assert_eq!(mixer.previous_track.as_ref().unwrap().filepath, "bgm/test1.mp3");
    assert_eq!(mixer.previous_track.as_ref().unwrap().volume, 0.5);

    // After 1 second, previous track should be 0.0 (removed), current should be 0.5
    mixer.update(1.0);
    assert!(mixer.previous_track.is_none());
    assert_eq!(mixer.current_track.as_ref().unwrap().volume, 0.5);
}

#[test]
fn test_play_same_track_twice() {
    let mut mixer = BgmCrossfadeMixer::new(2.0);
    mixer.play_music("bgm/test1.mp3");
    
    mixer.update(1.0);
    assert_eq!(mixer.current_track.as_ref().unwrap().volume, 0.5);

    // Play same track again
    mixer.play_music("bgm/test1.mp3");

    // Should not change anything
    assert!(mixer.previous_track.is_none());
    assert_eq!(mixer.current_track.as_ref().unwrap().filepath, "bgm/test1.mp3");
    assert_eq!(mixer.current_track.as_ref().unwrap().volume, 0.5);
}

#[test]
fn test_bgm_mixer_math_volume_levels_during_crossfade() {
    let mut mixer = BgmCrossfadeMixer::new(2.0); // 2 seconds fade duration
    
    // Start first track
    mixer.play_music("bgm/track_a.mp3");
    mixer.update(2.0); // fully fade in current track
    assert_eq!(mixer.current_track.as_ref().unwrap().volume, 1.0);
    
    // Play second track to trigger crossfade
    mixer.play_music("bgm/track_b.mp3");
    
    // At exactly time 0.0 (no update yet)
    assert_eq!(mixer.previous_track.as_ref().unwrap().volume, 1.0);
    assert_eq!(mixer.current_track.as_ref().unwrap().volume, 0.0);
    
    // Step 1.0s (halfway through crossfade)
    mixer.update(1.0);
    assert_eq!(mixer.previous_track.as_ref().unwrap().volume, 0.5);
    assert_eq!(mixer.current_track.as_ref().unwrap().volume, 0.5);
    
    // Step another 1.0s (crossfade complete)
    mixer.update(1.0);
    assert!(mixer.previous_track.is_none());
    assert_eq!(mixer.current_track.as_ref().unwrap().volume, 1.0);
}

#[test]
fn test_map_outdoor_environment_bgm_trigger() {
    use m2_client::map_outdoor_vfs_loader::MapOutdoorVfsLoader;
    
    let mut mixer = BgmCrossfadeMixer::new(1.0);
    let mut map_loader = MapOutdoorVfsLoader::new();
    
    // Manually setting environment to simulate MapSetting parsing
    map_loader.setting.environment = "desert".to_string();
    
    // Trigger auto play
    map_loader.auto_play_environment_bgm(&mut mixer);
    
    // Verify that the filepath is correctly formatted
    assert!(mixer.current_track.is_some());
    assert_eq!(mixer.current_track.as_ref().unwrap().filepath, "bgm/desert.mp3");
}

#[test]
fn test_global_volume_scaling() {
    let mut mixer = BgmCrossfadeMixer::new(2.0);
    mixer.set_global_volume(0.5);
    mixer.play_music("bgm/test1.mp3");
    
    // After 1 second, internal volume should be 0.5, effective should be 0.25
    mixer.update(1.0);
    assert_eq!(mixer.current_track.as_ref().unwrap().volume, 0.5);
    assert_eq!(mixer.get_current_track_effective_volume(), Some(0.25));

    // Play second track
    mixer.play_music("bgm/test2.mp3");

    // Previous track internal volume 0.5, effective 0.25
    assert_eq!(mixer.previous_track.as_ref().unwrap().volume, 0.5);
    assert_eq!(mixer.get_previous_track_effective_volume(), Some(0.25));
    
    // Current track internal volume 0.0, effective 0.0
    assert_eq!(mixer.current_track.as_ref().unwrap().volume, 0.0);
    assert_eq!(mixer.get_current_track_effective_volume(), Some(0.0));

    // After 1 second, previous track should be removed, current should be internal 0.5, effective 0.25
    mixer.update(1.0);
    assert!(mixer.previous_track.is_none());
    assert_eq!(mixer.get_previous_track_effective_volume(), None);
    assert_eq!(mixer.current_track.as_ref().unwrap().volume, 0.5);
    assert_eq!(mixer.get_current_track_effective_volume(), Some(0.25));
}
