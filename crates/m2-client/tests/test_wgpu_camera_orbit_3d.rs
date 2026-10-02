use m2_client::wgpu_camera_orbit_3d::{WgpuCameraOrbit3D, CAMERA_MIN_DISTANCE, CAMERA_MAX_DISTANCE, CAMERA_TARGET_STANDARD, CAMERA_TARGET_FACE};
use glam::Vec3;

#[test]
fn test_camera_rotation_limits() {
    let mut camera = WgpuCameraOrbit3D::new();
    camera.pitch = 30.0;
    camera.yaw = 0.0;

    // Pitch over the limit (should clamp to 80.0)
    camera.rotate(100.0, 0.0);
    assert_eq!(camera.pitch, 80.0);

    // Pitch under the limit (should clamp to 10.0)
    camera.rotate(-100.0, 0.0);
    assert_eq!(camera.pitch, 10.0);

    // Yaw wrap around
    camera.rotate(0.0, 400.0);
    assert!((camera.yaw - 40.0).abs() < f32::EPSILON);

    camera.rotate(0.0, -100.0);
    assert!((camera.yaw - 300.0).abs() < f32::EPSILON);
}

#[test]
fn test_camera_zoom_limits() {
    let mut camera = WgpuCameraOrbit3D::new();
    
    // Zoom out over the limit
    camera.zoom(5000.0);
    assert_eq!(camera.distance, CAMERA_MAX_DISTANCE);

    // Zoom in under the limit
    camera.zoom(-5000.0);
    assert_eq!(camera.distance, CAMERA_MIN_DISTANCE);
}

#[test]
fn test_camera_dynamic_target_height() {
    let mut camera = WgpuCameraOrbit3D::new();
    
    // At max distance, target height should be STANDARD
    camera.set_distance(CAMERA_MAX_DISTANCE);
    assert!((camera.get_dynamic_target_height() - CAMERA_TARGET_STANDARD).abs() < f32::EPSILON);
    
    // At min distance, target height should be FACE
    camera.set_distance(CAMERA_MIN_DISTANCE);
    assert!((camera.get_dynamic_target_height() - CAMERA_TARGET_FACE).abs() < f32::EPSILON);
    
    // Somewhere in between
    camera.set_distance((CAMERA_MAX_DISTANCE + CAMERA_MIN_DISTANCE) / 2.0);
    let expected = (CAMERA_TARGET_STANDARD + CAMERA_TARGET_FACE) / 2.0;
    assert!((camera.get_dynamic_target_height() - expected).abs() < f32::EPSILON);
}

#[test]
fn test_camera_terrain_collision() {
    let mut camera = WgpuCameraOrbit3D::new();
    camera.process_terrain_collision = true;
    camera.terrain_collision_radius = 10.0;
    
    // Place target such that eye will be well below terrain
    let target_pos = Vec3::new(0.0, 0.0, 0.0);
    
    // Terrain height function always returns 100.0
    let terrain_fn = |_, _| 100.0;

    camera.update(target_pos, terrain_fn);

    // After update, eye_pos.z should be terrain_height + 2 * collision_radius
    let expected_z = 100.0 + 2.0 * camera.terrain_collision_radius * camera.up.z;
    assert!(camera.eye_pos.z >= expected_z - 0.01, "Expected z >= {}, got {}", expected_z, camera.eye_pos.z);
}

#[test]
fn test_camera_matrices() {
    let mut camera = WgpuCameraOrbit3D::new();
    camera.update(Vec3::ZERO, |_, _| 0.0);

    let view = camera.get_view_matrix();
    let proj = camera.get_projection_matrix();

    // Check if matrices are not empty (identity matrices have diagonal elements = 1)
    // Actually perspective matrix will have non-1 values in many places.
    // We just check they are generated and not all zeroes.
    assert!(view[0][0] != 0.0 || view[1][1] != 0.0 || view[2][2] != 0.0);
    assert!(proj[0][0] != 0.0 || proj[1][1] != 0.0 || proj[2][2] != 0.0 || proj[3][3] != 0.0);
}

#[test]
fn test_camera_ray_unproject() {
    let mut camera = WgpuCameraOrbit3D::new();
    camera.update(Vec3::ZERO, |_, _| 0.0);

    // Screen dimensions
    let w = 800.0;
    let h = 600.0;

    // Center of screen
    let ray = camera.unproject_ray(w / 2.0, h / 2.0, w, h);

    // Direction should be roughly the same as view direction
    let dot = ray.direction.dot(camera.view);
    assert!(dot > 0.99, "Expected ray direction close to view direction. Dot product: {}", dot);

    // Invalid screen dimensions should fallback safely
    let bad_ray = camera.unproject_ray(w / 2.0, h / 2.0, 0.0, 0.0);
    assert_eq!(bad_ray.direction.z, 1.0);
}
