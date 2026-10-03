use glam::Vec3;

#[path = "../src/spatial_audio_3d_panner.rs"]
mod spatial_audio_3d_panner;
use spatial_audio_3d_panner::SpatialAudioPanner;

#[test]
fn test_sound_at_exact_camera_position() {
    let panner = SpatialAudioPanner::new(100.0);
    
    let listener_pos = Vec3::new(0.0, 0.0, 0.0);
    let listener_forward = Vec3::new(0.0, 0.0, -1.0);
    let listener_up = Vec3::new(0.0, 1.0, 0.0);
    let source_pos = Vec3::new(0.0, 0.0, 0.0); // Exact same position
    
    let result = panner.calculate(listener_pos, listener_forward, listener_up, source_pos, 1.0);
    
    assert_eq!(result.total_volume, 1.0, "Total volume should be max at zero distance");
    assert_eq!(result.pan, 0.0, "Pan should be perfectly centered when distance is 0");
    
    // Equal power panning at center gives ~0.707 for both channels
    let center_val = std::f32::consts::FRAC_PI_4.cos();
    assert!((result.left_volume - center_val).abs() < 1e-5);
    assert!((result.right_volume - center_val).abs() < 1e-5);
}

#[test]
fn test_sound_max_distance_attenuation() {
    let panner = SpatialAudioPanner::new(100.0).with_reference_distance(10.0);
    
    let listener_pos = Vec3::new(0.0, 0.0, 0.0);
    let listener_forward = Vec3::new(0.0, 0.0, -1.0);
    let listener_up = Vec3::new(0.0, 1.0, 0.0);
    
    // Within reference distance
    let res1 = panner.calculate(listener_pos, listener_forward, listener_up, Vec3::new(0.0, 0.0, -5.0), 1.0);
    assert_eq!(res1.total_volume, 1.0, "Volume should be 1.0 within reference distance");
    
    // Halfway between ref and max distance: dist = 55.0. 
    // formula: 1.0 - ((55 - 10) / (100 - 10)) = 1.0 - (45 / 90) = 0.5
    let res2 = panner.calculate(listener_pos, listener_forward, listener_up, Vec3::new(0.0, 0.0, -55.0), 1.0);
    assert!((res2.total_volume - 0.5).abs() < 1e-5, "Volume should be 0.5 halfway through falloff");
    
    // At max distance
    let res3 = panner.calculate(listener_pos, listener_forward, listener_up, Vec3::new(0.0, 0.0, -100.0), 1.0);
    assert_eq!(res3.total_volume, 0.0, "Volume should be 0.0 at max distance");
    
    // Beyond max distance
    let res4 = panner.calculate(listener_pos, listener_forward, listener_up, Vec3::new(0.0, 0.0, -150.0), 1.0);
    assert_eq!(res4.total_volume, 0.0, "Volume should remain 0.0 beyond max distance");
}

#[test]
fn test_sound_explicitly_left() {
    let panner = SpatialAudioPanner::new(100.0);
    
    let listener_pos = Vec3::new(0.0, 0.0, 0.0);
    let listener_forward = Vec3::new(0.0, 0.0, -1.0);
    let listener_up = Vec3::new(0.0, 1.0, 0.0);
    // Right vector will be (1, 0, 0)
    // Put sound at (-10, 0, 0) which is directly left
    let source_pos = Vec3::new(-10.0, 0.0, 0.0);
    
    let result = panner.calculate(listener_pos, listener_forward, listener_up, source_pos, 1.0);
    
    assert!((result.pan - (-1.0)).abs() < 1e-5, "Pan should be -1.0 for directly left");
    assert!(result.left_volume > result.right_volume, "Left volume should be greater than right");
    assert!((result.left_volume - result.total_volume).abs() < 1e-5, "Left volume should be max (equal power with angle 0 -> cos(0)=1)");
    assert!(result.right_volume < 1e-5, "Right volume should be practically 0 (equal power with angle 0 -> sin(0)=0)");
}

#[test]
fn test_sound_explicitly_right() {
    let panner = SpatialAudioPanner::new(100.0);
    
    let listener_pos = Vec3::new(0.0, 0.0, 0.0);
    let listener_forward = Vec3::new(0.0, 0.0, -1.0);
    let listener_up = Vec3::new(0.0, 1.0, 0.0);
    // Right vector will be (1, 0, 0)
    // Put sound at (10, 0, 0) which is directly right
    let source_pos = Vec3::new(10.0, 0.0, 0.0);
    
    let result = panner.calculate(listener_pos, listener_forward, listener_up, source_pos, 1.0);
    
    assert!((result.pan - 1.0).abs() < 1e-5, "Pan should be 1.0 for directly right");
    assert!(result.right_volume > result.left_volume, "Right volume should be greater than left");
    assert!((result.right_volume - result.total_volume).abs() < 1e-5, "Right volume should be max");
    assert!(result.left_volume < 1e-5, "Left volume should be practically 0");
}

#[test]
fn test_sound_front_and_back() {
    let panner = SpatialAudioPanner::new(100.0);
    
    let listener_pos = Vec3::new(0.0, 0.0, 0.0);
    let listener_forward = Vec3::new(0.0, 0.0, -1.0);
    let listener_up = Vec3::new(0.0, 1.0, 0.0);
    
    // Front
    let res_front = panner.calculate(listener_pos, listener_forward, listener_up, Vec3::new(0.0, 0.0, -10.0), 1.0);
    assert!((res_front.pan - 0.0).abs() < 1e-5, "Pan should be 0.0 for directly in front");
    assert!((res_front.left_volume - res_front.right_volume).abs() < 1e-5, "L/R volume should be equal for front");
    
    // Back
    let res_back = panner.calculate(listener_pos, listener_forward, listener_up, Vec3::new(0.0, 0.0, 10.0), 1.0);
    assert!((res_back.pan - 0.0).abs() < 1e-5, "Pan should be 0.0 for directly behind");
    assert!((res_back.left_volume - res_back.right_volume).abs() < 1e-5, "L/R volume should be equal for back");
}
