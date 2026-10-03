#[path = "../src/camera_controller_complete.rs"]
mod camera_controller_complete;

use camera_controller_complete::{
    CameraControllerComplete, CameraState, CAMERA_MAX_DISTANCE, CAMERA_MIN_DISTANCE,
    CAMERA_TARGET_FACE, CAMERA_TARGET_STANDARD,
};
use glam::Vec3;

#[test]
fn test_camera_initialization() {
    let camera = CameraControllerComplete::new();
    assert_eq!(camera.distance, 1500.0);
    assert_eq!(camera.pitch, 30.0);
    assert_eq!(camera.roll, 0.0);
    assert_eq!(camera.camera_state, CameraState::Normal);
    assert_eq!(camera.target, Vec3::ZERO);
}

#[test]
fn test_camera_zoom_limits() {
    let mut camera = CameraControllerComplete::new();
    
    // Test zooming out to max distance
    camera.set_distance(CAMERA_MAX_DISTANCE + 1000.0);
    camera.update(|_, _| 0.0);
    assert_eq!(camera.distance, CAMERA_MAX_DISTANCE);
    
    // Test zooming in to min distance
    camera.set_distance(CAMERA_MIN_DISTANCE - 100.0);
    camera.update(|_, _| 0.0);
    assert_eq!(camera.distance, CAMERA_MIN_DISTANCE);
}

#[test]
fn test_camera_rotation_limits() {
    let mut camera = CameraControllerComplete::new();
    
    // Pitch limit test (should not exceed 80)
    camera.pitch(60.0); // 30 + 60 = 90, should clamp to 80
    assert_eq!(camera.pitch, 80.0);
    
    // Pitch limit test (should not go below -80)
    camera.pitch(-170.0); // 80 - 170 = -90, should clamp to -80
    assert_eq!(camera.pitch, -80.0);
    
    // Roll wrap test
    camera.roll(400.0);
    assert!(camera.roll > 0.0 && camera.roll <= 360.0); // 400 - 360 = 40.0
}

#[test]
fn test_camera_movement() {
    let mut camera = CameraControllerComplete::new();
    let initial_target = camera.target;
    let initial_eye = camera.eye;
    
    camera.move_vertical(50.0);
    assert_eq!(camera.target.z, initial_target.z + 50.0);
    assert_eq!(camera.eye.z, initial_eye.z + 50.0);
    
    let target_before = camera.target;
    camera.move_along_view(100.0);
    assert!(camera.target != target_before);
    
    let target_before_cross = camera.target;
    camera.move_along_cross(100.0);
    assert!(camera.target != target_before_cross);
}

#[test]
fn test_terrain_collision() {
    let mut camera = CameraControllerComplete::new();
    
    // Set target at Z=0
    camera.target = Vec3::new(0.0, 0.0, 0.0);
    // Put camera at Z=10
    camera.eye = Vec3::new(0.0, -100.0, 10.0);
    camera.set_view_matrix();
    
    // Provide a terrain height that is higher than the camera eye, e.g., 50.0
    // The camera should be pushed up.
    camera.update(|_, _| 50.0);
    
    assert_eq!(camera.camera_state, CameraState::CantGoDown);
    assert!(camera.eye.z >= 50.0);
}

#[test]
fn test_angular_velocity_smoothing() {
    let mut camera = CameraControllerComplete::new();
    
    camera.angular_velocity = Vec3::new(10.0, 20.0, 30.0);
    camera.update(|_, _| 0.0);
    
    // Should be halved
    assert_eq!(camera.angular_velocity.x, 5.0);
    assert_eq!(camera.angular_velocity.y, 10.0);
    assert_eq!(camera.angular_velocity.z, 15.0);
    
    // Do it enough times to make it near zero and snap to 0
    for _ in 0..10 {
        camera.update(|_, _| 0.0);
    }
    
    assert_eq!(camera.angular_velocity.x, 0.0);
    assert_eq!(camera.angular_velocity.y, 0.0);
    assert_eq!(camera.angular_velocity.z, 0.0);
}

#[test]
fn test_dynamic_target_height() {
    let mut camera = CameraControllerComplete::new();
    
    // Max distance (2500)
    camera.set_distance(CAMERA_MAX_DISTANCE);
    camera.update(|_, _| 0.0);
    // Target height should be Standard (100.0)
    assert_eq!(camera.target_height, CAMERA_TARGET_STANDARD);
    
    // Min distance (200)
    camera.set_distance(CAMERA_MIN_DISTANCE);
    camera.update(|_, _| 0.0);
    // Target height should be Face (150.0)
    assert_eq!(camera.target_height, CAMERA_TARGET_FACE);
}
