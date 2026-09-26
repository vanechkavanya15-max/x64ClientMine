#[path = "../src/sound_ambience_manager.rs"]
mod sound_ambience_manager;

use sound_ambience_manager::SoundAmbienceManager;

#[test]
fn test_ambience_zone_full_volume_at_center() {
    let mut manager = SoundAmbienceManager::new();
    manager.add_ambience_zone(1, "waterfall", (10.0, 20.0, 30.0), 100.0, 1.0);
    manager.update_listener_position((10.0, 20.0, 30.0));
    
    let vol = manager.get_zone_volume(1);
    assert!((vol - 1.0).abs() < f32::EPSILON, "Volume should be max at center");
}

#[test]
fn test_ambience_zone_volume_attenuation() {
    let mut manager = SoundAmbienceManager::new();
    manager.add_ambience_zone(1, "waterfall", (0.0, 0.0, 0.0), 100.0, 1.0);
    
    // Distance = 50.0 (halfway)
    manager.update_listener_position((50.0, 0.0, 0.0));
    let vol_half = manager.get_zone_volume(1);
    assert!((vol_half - 0.5).abs() < 1e-4, "Volume should be 50% at half distance, got {}", vol_half);

    // Distance = 75.0 (75% to edge)
    manager.update_listener_position((0.0, 75.0, 0.0));
    let vol_quarter = manager.get_zone_volume(1);
    assert!((vol_quarter - 0.25).abs() < 1e-4, "Volume should be 25% at 75% distance, got {}", vol_quarter);
}

#[test]
fn test_ambience_zone_zero_volume_outside_radius() {
    let mut manager = SoundAmbienceManager::new();
    manager.add_ambience_zone(1, "waterfall", (0.0, 0.0, 0.0), 100.0, 1.0);
    
    // Exactly at edge
    manager.update_listener_position((100.0, 0.0, 0.0));
    let vol_edge = manager.get_zone_volume(1);
    assert!(vol_edge < 1e-4, "Volume should be 0 at exact edge radius, got {}", vol_edge);

    // Outside edge
    manager.update_listener_position((150.0, 0.0, 0.0));
    let vol_outside = manager.get_zone_volume(1);
    assert!(vol_outside < f32::EPSILON, "Volume should be 0 outside radius");
}

#[test]
fn test_multiple_independent_ambience_zones() {
    let mut manager = SoundAmbienceManager::new();
    manager.add_ambience_zone(1, "waterfall", (0.0, 0.0, 0.0), 100.0, 1.0);
    manager.add_ambience_zone(2, "birds", (200.0, 0.0, 0.0), 100.0, 0.8);

    // Move to zone 1 center
    manager.update_listener_position((0.0, 0.0, 0.0));
    assert!((manager.get_zone_volume(1) - 1.0).abs() < f32::EPSILON);
    assert!(manager.get_zone_volume(2) < f32::EPSILON);

    // Move to zone 2 center
    manager.update_listener_position((200.0, 0.0, 0.0));
    assert!(manager.get_zone_volume(1) < f32::EPSILON);
    assert!((manager.get_zone_volume(2) - 0.8).abs() < f32::EPSILON);

    // Move midway
    manager.update_listener_position((100.0, 0.0, 0.0));
    // Both are at their very edges (distance 100 from centers)
    assert!(manager.get_zone_volume(1) < 1e-4);
    assert!(manager.get_zone_volume(2) < 1e-4);
}

#[test]
fn test_global_ambience() {
    let mut manager = SoundAmbienceManager::new();
    manager.add_ambience_zone(1, "wind", (0.0, 0.0, 0.0), 0.0, 1.0);

    let initial_vol = manager.get_zone_volume(1);
    
    // Global ambience should vary slightly with listener updates (time sim)
    manager.update_listener_position((100.0, 100.0, 100.0));
    let next_vol = manager.get_zone_volume(1);
    
    assert!((next_vol - 1.0).abs() <= 0.11, "Global volume variation should be clamped near max_vol");
    assert!(initial_vol != next_vol || (initial_vol - 1.0).abs() < f32::EPSILON, "Global volume should vary");
}

#[test]
fn test_get_invalid_zone() {
    let manager = SoundAmbienceManager::new();
    assert_eq!(manager.get_zone_volume(999), 0.0);
}
