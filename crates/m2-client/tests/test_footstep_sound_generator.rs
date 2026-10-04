use m2_client::engine_footstep_sound_generator::{FootstepSoundGenerator, FootstepSurfaceType, ATTRIBUTE_WATER};

#[test]
fn test_get_surface_type() {
    // Test attribute override
    assert_eq!(
        FootstepSoundGenerator::get_surface_type(ATTRIBUTE_WATER, 0),
        FootstepSurfaceType::Water
    );

    // Test tiles
    assert_eq!(
        FootstepSoundGenerator::get_surface_type(0, 3),
        FootstepSurfaceType::Grass
    );
    assert_eq!(
        FootstepSoundGenerator::get_surface_type(0, 8),
        FootstepSurfaceType::Stone
    );
    assert_eq!(
        FootstepSoundGenerator::get_surface_type(0, 13),
        FootstepSurfaceType::Mud
    );
    assert_eq!(
        FootstepSoundGenerator::get_surface_type(0, 18),
        FootstepSurfaceType::Wood
    );
    assert_eq!(
        FootstepSoundGenerator::get_surface_type(0, 23),
        FootstepSurfaceType::Grass
    );
    
    // Default fallback
    assert_eq!(
        FootstepSoundGenerator::get_surface_type(0, 99),
        FootstepSurfaceType::Stone
    );
}

#[test]
fn test_generate_footstep_sound_path() {
    // Generate for PC on Grass
    let path_pc_grass = FootstepSoundGenerator::generate_footstep_sound_path(100.0, 200.0, 0, 0, 12345, true);
    assert!(path_pc_grass.starts_with("sound/pc/walk/walk_grass_"));
    assert!(path_pc_grass.ends_with(".wav"));
    
    // Generate for Mob on Water
    let path_mob_water = FootstepSoundGenerator::generate_footstep_sound_path(100.0, 200.0, ATTRIBUTE_WATER, 0, 12345, false);
    assert!(path_mob_water.starts_with("sound/mob/walk/walk_water_"));
    assert!(path_mob_water.ends_with(".wav"));
    
    // Generate for PC on Wood
    let path_pc_wood = FootstepSoundGenerator::generate_footstep_sound_path(50.0, 50.0, 0, 17, 98765, true);
    assert!(path_pc_wood.starts_with("sound/pc/walk/walk_wood_"));
}

#[test]
fn test_deterministic_variants() {
    // Test that the same inputs yield the same sound file exactly
    let path1 = FootstepSoundGenerator::generate_footstep_sound_path(150.5, 300.2, 0, 7, 5000, true);
    let path2 = FootstepSoundGenerator::generate_footstep_sound_path(150.5, 300.2, 0, 7, 5000, true);
    
    assert_eq!(path1, path2);
}

#[test]
fn test_process_animation_footstep_sync() {
    // Test match
    let result_match = FootstepSoundGenerator::process_animation_footstep(10, 10, 100.0, 100.0, 0, 0, 1000, true);
    assert!(result_match.is_some());
    assert!(result_match.unwrap().starts_with("sound/pc/walk/walk_grass_"));
    
    // Test mismatch
    let result_mismatch = FootstepSoundGenerator::process_animation_footstep(9, 10, 100.0, 100.0, 0, 0, 1000, true);
    assert!(result_mismatch.is_none());
}
