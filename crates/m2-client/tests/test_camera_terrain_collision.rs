#[path = "../src/camera_terrain_collision.rs"]
mod camera_terrain_collision;

use camera_terrain_collision::resolve_camera_terrain_collision;

#[test]
fn test_camera_above_ground() {
    // Ground is at z = 10.0, eye is at z = 100.0, which is > 10.0 + 50.0 (60.0).
    // No collision should occur.
    let (new_x, new_y, new_z, new_dist) = resolve_camera_terrain_collision(
        0.0, 0.0, 100.0,
        1.0, 0.0, 0.0,
        200.0,
        |_x, _y| 10.0,
    );

    assert_eq!(new_x, 0.0);
    assert_eq!(new_y, 0.0);
    assert_eq!(new_z, 100.0);
    assert_eq!(new_dist, 200.0);
}

#[test]
fn test_camera_below_clearance() {
    // Ground is at z = 20.0, eye is at z = 50.0.
    // Minimum clearance is 50.0, so target Z is 20.0 + 50.0 = 70.0.
    // Collision should elevate the camera to z = 70.0.
    let (new_x, new_y, new_z, _new_dist) = resolve_camera_terrain_collision(
        0.0, 0.0, 50.0,
        0.0, 1.0, 0.0,
        100.0,
        |_x, _y| 20.0,
    );

    assert_eq!(new_x, 0.0);
    assert_eq!(new_y, 0.0);
    assert_eq!(new_z, 70.0); // 20.0 (ground) + 50.0 (clearance)
}

#[test]
fn test_camera_smooth_zoom_correction() {
    // When camera Z is elevated, distance should be recalculated so that the target 
    // position remains consistent (zoom correction).
    // Eye at (0, 0, 0).
    // View dir = (1, 0, 0), so looking perfectly forward along X axis.
    // Distance = 100.0. Target is at (100, 0, 0).
    // Ground is at z = 0.0.
    // Minimum clearance is 50.0, target z is 50.0.
    // Eye Z elevates to 50.0.
    // New distance to target (100, 0, 0) from new eye (0, 0, 50) is sqrt(100^2 + 50^2) = sqrt(12500) ≈ 111.8034.
    
    let (new_x, new_y, new_z, new_dist) = resolve_camera_terrain_collision(
        0.0, 0.0, 0.0,
        1.0, 0.0, 0.0,
        100.0,
        |_x, _y| 0.0,
    );

    assert_eq!(new_x, 0.0);
    assert_eq!(new_y, 0.0);
    assert_eq!(new_z, 50.0);
    
    let expected_dist = (100.0_f32.powi(2) + 50.0_f32.powi(2)).sqrt();
    assert!((new_dist - expected_dist).abs() < 1e-4);
}

#[test]
fn test_camera_below_ground() {
    // Ground is at z = 200.0, eye is at z = 100.0 (literally under the map).
    // Target Z is 250.0.
    let (new_x, new_y, new_z, _) = resolve_camera_terrain_collision(
        10.0, -10.0, 100.0,
        0.0, 0.0, -1.0,
        300.0,
        |_x, _y| 200.0,
    );

    assert_eq!(new_x, 10.0);
    assert_eq!(new_y, -10.0);
    assert_eq!(new_z, 250.0);
}

#[test]
fn test_camera_no_view_dir() {
    // If view_dir is 0 length, it shouldn't recalculate zoom or shouldn't crash.
    let (_new_x, _new_y, new_z, new_dist) = resolve_camera_terrain_collision(
        0.0, 0.0, 0.0,
        0.0, 0.0, 0.0,
        100.0,
        |_x, _y| 10.0,
    );

    assert_eq!(new_z, 60.0);
    assert_eq!(new_dist, 100.0); // No zoom correction
}
