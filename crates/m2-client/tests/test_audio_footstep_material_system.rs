use m2_client::audio_footstep_material_system::{AudioFootstepMaterialSystem, FootstepSurfaceType, ATTRIBUTE_WATER};

#[test]
fn test_get_surface_type_water() {
    let surface = AudioFootstepMaterialSystem::get_surface_type(ATTRIBUTE_WATER, 0);
    assert_eq!(surface, FootstepSurfaceType::Water);

    let surface_with_tile = AudioFootstepMaterialSystem::get_surface_type(ATTRIBUTE_WATER, 12);
    assert_eq!(surface_with_tile, FootstepSurfaceType::Water);
}

#[test]
fn test_get_surface_type_grass() {
    let surface1 = AudioFootstepMaterialSystem::get_surface_type(0, 0);
    assert_eq!(surface1, FootstepSurfaceType::Grass);

    let surface2 = AudioFootstepMaterialSystem::get_surface_type(0, 5);
    assert_eq!(surface2, FootstepSurfaceType::Grass);
}

#[test]
fn test_get_surface_type_stone() {
    let surface1 = AudioFootstepMaterialSystem::get_surface_type(0, 6);
    assert_eq!(surface1, FootstepSurfaceType::Stone);

    let surface2 = AudioFootstepMaterialSystem::get_surface_type(0, 15);
    assert_eq!(surface2, FootstepSurfaceType::Stone);
}

#[test]
fn test_get_surface_type_dirt() {
    let surface1 = AudioFootstepMaterialSystem::get_surface_type(0, 16);
    assert_eq!(surface1, FootstepSurfaceType::Dirt);

    let surface2 = AudioFootstepMaterialSystem::get_surface_type(0, 25);
    assert_eq!(surface2, FootstepSurfaceType::Dirt);
}

#[test]
fn test_get_surface_type_fallback() {
    // Default fallback to Grass
    let surface = AudioFootstepMaterialSystem::get_surface_type(0, 100);
    assert_eq!(surface, FootstepSurfaceType::Grass);
}

#[test]
fn test_trigger_footstep_event_variants() {
    // We expect 4 variants, we can try different time_ms to see if we get different variants
    let mut variants_seen = [false; 5]; // 1-based index (1..4)
    
    for time_ms in 0..100 {
        let sound = AudioFootstepMaterialSystem::trigger_footstep_event(100.0, 200.0, 0, 0, time_ms);
        assert!(sound.starts_with("sound/common/walk/walk_grass_"));
        assert!(sound.ends_with(".wav"));
        
        let variant_str = &sound[sound.len() - 6..sound.len() - 4];
        let variant: usize = variant_str.parse().unwrap();
        
        assert!(variant >= 1 && variant <= 4);
        variants_seen[variant] = true;
    }
    
    // We should have seen multiple variants over 100 iterations
    let count = variants_seen.iter().filter(|&&v| v).count();
    assert!(count > 1, "Should generate multiple pseudo-random variants");
}

#[test]
fn test_trigger_footstep_event_surface_prefixes() {
    let sound_water = AudioFootstepMaterialSystem::trigger_footstep_event(0.0, 0.0, ATTRIBUTE_WATER, 0, 0);
    assert!(sound_water.starts_with("sound/common/walk/walk_water"));

    let sound_grass = AudioFootstepMaterialSystem::trigger_footstep_event(0.0, 0.0, 0, 0, 0);
    assert!(sound_grass.starts_with("sound/common/walk/walk_grass"));

    let sound_stone = AudioFootstepMaterialSystem::trigger_footstep_event(0.0, 0.0, 0, 10, 0);
    assert!(sound_stone.starts_with("sound/common/walk/walk_stone"));

    let sound_dirt = AudioFootstepMaterialSystem::trigger_footstep_event(0.0, 0.0, 0, 20, 0);
    assert!(sound_dirt.starts_with("sound/common/walk/walk_dirt"));
}
