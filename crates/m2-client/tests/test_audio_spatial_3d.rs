use m2_client::audio_spatial_3d_sound_engine::AudioSpatial3DEngine;

#[test]
fn test_spatial_distance_attenuation() {
    let mut engine = AudioSpatial3DEngine::new();
    engine.set_listener_position(0.0, 0.0, 0.0);
    engine.set_listener_orientation(0.0, 0.0, -1.0, 0.0, 1.0, 0.0);

    let min_dist = 100.0;
    let max_dist = 5000.0;

    // Within min_dist, volume should be 1.0
    let (vol1, _) = engine.calculate_spatial_params(0.0, 0.0, -50.0, min_dist, max_dist);
    assert_eq!(vol1, 1.0);

    // Halfway between min_dist and max_dist, volume should be 0.5
    let half_dist = (min_dist + max_dist) / 2.0;
    let (vol2, _) = engine.calculate_spatial_params(0.0, 0.0, -half_dist, min_dist, max_dist);
    assert!((vol2 - 0.5).abs() < 1e-4);

    // At or beyond max_dist, volume should be 0.0
    let (vol3, _) = engine.calculate_spatial_params(0.0, 0.0, -max_dist, min_dist, max_dist);
    assert_eq!(vol3, 0.0);
    
    let (vol4, _) = engine.calculate_spatial_params(0.0, 0.0, -max_dist * 2.0, min_dist, max_dist);
    assert_eq!(vol4, 0.0);
}

#[test]
fn test_spatial_stereo_pan_right_channel() {
    let mut engine = AudioSpatial3DEngine::new();
    engine.set_listener_position(0.0, 0.0, 0.0);
    // Looking forward (-z), Up is +y. So Right is +x.
    engine.set_listener_orientation(0.0, 0.0, -1.0, 0.0, 1.0, 0.0);

    let min_dist = 100.0;
    let max_dist = 5000.0;

    // Source directly to the right (+x axis)
    let (_, pan) = engine.calculate_spatial_params(500.0, 0.0, 0.0, min_dist, max_dist);
    assert!((pan - 1.0).abs() < 1e-4, "Pan should be fully right (1.0), got {}", pan);
}

#[test]
fn test_spatial_stereo_pan_left_channel() {
    let mut engine = AudioSpatial3DEngine::new();
    engine.set_listener_position(0.0, 0.0, 0.0);
    // Looking forward (-z), Up is +y. So Right is +x. Left is -x.
    engine.set_listener_orientation(0.0, 0.0, -1.0, 0.0, 1.0, 0.0);

    let min_dist = 100.0;
    let max_dist = 5000.0;

    // Source directly to the left (-x axis)
    let (_, pan) = engine.calculate_spatial_params(-500.0, 0.0, 0.0, min_dist, max_dist);
    assert!((pan - (-1.0)).abs() < 1e-4, "Pan should be fully left (-1.0), got {}", pan);
}

#[test]
fn test_spatial_stereo_pan_center() {
    let mut engine = AudioSpatial3DEngine::new();
    engine.set_listener_position(0.0, 0.0, 0.0);
    // Looking forward (-z), Up is +y.
    engine.set_listener_orientation(0.0, 0.0, -1.0, 0.0, 1.0, 0.0);

    let min_dist = 100.0;
    let max_dist = 5000.0;

    // Source directly in front (-z axis)
    let (_, pan1) = engine.calculate_spatial_params(0.0, 0.0, -500.0, min_dist, max_dist);
    assert!((pan1 - 0.0).abs() < 1e-4, "Pan should be center (0.0), got {}", pan1);

    // Source directly behind (+z axis)
    let (_, pan2) = engine.calculate_spatial_params(0.0, 0.0, 500.0, min_dist, max_dist);
    assert!((pan2 - 0.0).abs() < 1e-4, "Pan should be center (0.0), got {}", pan2);

    // Source directly above (+y axis)
    let (_, pan3) = engine.calculate_spatial_params(0.0, 500.0, 0.0, min_dist, max_dist);
    assert!((pan3 - 0.0).abs() < 1e-4, "Pan should be center (0.0), got {}", pan3);
}

#[test]
fn test_distant_monster_volume_and_pan() {
    let mut engine = AudioSpatial3DEngine::new();
    
    // Player at (100, 50, 200), looking slightly to the right (x=1, z=-1)
    engine.set_listener_position(100.0, 50.0, 200.0);
    engine.set_listener_orientation(1.0, 0.0, -1.0, 0.0, 1.0, 0.0);
    
    // Distant monster
    // Forward is (0.707, 0, -0.707)
    // Up is (0, 1, 0)
    // Right = CrossProduct(Forward, Up)
    // right_x = f_y*u_z - f_z*u_y = 0 - (-0.707)*1 = 0.707
    // right_y = f_z*u_x - f_x*u_z = 0 - 0 = 0
    // right_z = f_x*u_y - f_y*u_x = 0.707*1 - 0 = 0.707
    // Right is roughly (0.707, 0, 0.707)
    
    // Let's put a monster exactly along the Right vector from the player
    let min_dist = 100.0;
    let max_dist = 5000.0;
    
    let dist = 2550.0; // Halfway between min and max
    let right_x = 1.0 / (2.0f32).sqrt();
    let right_z = 1.0 / (2.0f32).sqrt();
    
    let monster_x = 100.0 + right_x * dist;
    let monster_y = 50.0;
    let monster_z = 200.0 + right_z * dist;
    
    let (vol, pan) = engine.calculate_spatial_params(monster_x, monster_y, monster_z, min_dist, max_dist);
    
    // Volume should be exactly 0.5 because it's halfway between 100 and 5000
    assert!((vol - 0.5).abs() < 1e-4, "Volume should be 0.5, got {}", vol);
    
    // Pan should be exactly 1.0 because it's perfectly on the Right axis
    assert!((pan - 1.0).abs() < 1e-4, "Pan should be fully right (1.0), got {}", pan);
}
