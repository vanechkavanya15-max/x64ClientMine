use glam::{Vec3, Mat4};
use m2_client::wgpu_camera_orbit_controller::{CameraOrbitController, CameraState};

#[test]
fn test_default_initialization() {
    let camera = CameraOrbitController::new();
    assert_eq!(camera.distance, 1500.0);
    assert_eq!(camera.pitch, 30.0);
    assert_eq!(camera.yaw, 0.0);
    assert_eq!(camera.camera_state, CameraState::Normal);
    assert_ne!(camera.view_matrix, Mat4::IDENTITY);
    assert_ne!(camera.proj_matrix, Mat4::IDENTITY);
}

#[test]
fn test_zoom_clamp() {
    let mut camera = CameraOrbitController::new();
    
    // Zoom in beyond min distance
    camera.zoom(0.1); 
    assert_eq!(camera.distance, 400.0); // MIN_DISTANCE
    
    // Zoom out beyond max distance
    camera.zoom(100.0);
    assert_eq!(camera.distance, 2500.0); // MAX_DISTANCE
}

#[test]
fn test_rotation_pitch_clamp() {
    let mut camera = CameraOrbitController::new();
    camera.pitch = 30.0;
    
    // Try to pitch up beyond +80
    camera.pitch(60.0);
    assert_eq!(camera.pitch, 80.0);
    
    // Try to pitch down beyond 10
    camera.pitch(-180.0);
    assert_eq!(camera.pitch, 10.0);
}

#[test]
fn test_rotation_yaw_wrap() {
    let mut camera = CameraOrbitController::new();
    camera.yaw = 350.0;
    
    // Wrap around 360
    camera.roll(20.0);
    assert_eq!(camera.yaw, 10.0);
    
    // Wrap around below 0
    camera.yaw = 10.0;
    camera.roll(-20.0);
    assert_eq!(camera.yaw, 350.0); // -10 + 360 = 350
}

#[test]
fn test_target_height_calculation() {
    let mut camera = CameraOrbitController::new();
    
    // Target height is now strictly Z = 150.0
    camera.distance = 1500.0;
    camera.update(Vec3::ZERO, |_, _| 0.0);
    assert!((camera.target_pos.z - 150.0).abs() < 0.01);
    
    // At max distance (2500), height should still be 150.0
    camera.distance = 2500.0;
    camera.update(Vec3::ZERO, |_, _| 0.0);
    assert!((camera.target_pos.z - 150.0).abs() < 0.01);
    
    // At min distance (400), height should still be 150.0
    camera.distance = 400.0;
    camera.update(Vec3::ZERO, |_, _| 0.0);
    assert!((camera.target_pos.z - 150.0).abs() < 0.01);
}

#[test]
fn test_terrain_collision_shortens_distance() {
    let mut camera = CameraOrbitController::new();
    camera.distance = 1000.0;
    camera.pitch = 0.0; // Horizontal looking
    camera.yaw = 0.0;
    camera.terrain_collision_radius = 10.0;
    
    // Original eye_pos without terrain: (0, -1000, ~134.78)
    // We will set a high terrain block at y = -500 to simulate a wall
    let terrain_fn = |_x: f32, y: f32| -> f32 {
        if y < -400.0 && y > -600.0 {
            return 500.0; // High mountain
        }
        0.0 // Flat ground
    };
    
    camera.update(Vec3::ZERO, terrain_fn);
    
    assert_eq!(camera.camera_state, CameraState::CantGoDown);
    // distance should be much shorter than 1000 now
    assert!(camera.distance < 1000.0);
}

#[test]
fn test_no_terrain_collision_on_flat_ground() {
    let mut camera = CameraOrbitController::new();
    camera.distance = 1000.0;
    camera.pitch = 30.0;
    camera.target_pos = Vec3::new(0.0, 0.0, 100.0);
    
    // Z is high above flat ground
    let terrain_fn = |_, _| -1000.0; // Always flat ground far below
    
    camera.update(Vec3::new(0.0, 0.0, 100.0), terrain_fn);
    
    assert_eq!(camera.camera_state, CameraState::Normal);
    assert_eq!(camera.distance, 1000.0);
}

#[test]
fn test_matrices_recomputed() {
    let mut camera = CameraOrbitController::new();
    let initial_view = camera.view_matrix;
    
    // Change pitch and distance
    camera.pitch(10.0);
    camera.zoom(0.5);
    camera.update(Vec3::new(100.0, 100.0, 0.0), |_, _| 0.0);
    
    assert_ne!(camera.view_matrix, initial_view);
}