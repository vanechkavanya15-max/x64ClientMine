use m2_client::camera_orbit_3d::OrbitCamera;

#[test]
fn test_camera_eye_zero_yaw_pitch() {
    let camera = OrbitCamera::new(
        (100.0, 200.0, 300.0),
        0.0,
        0.0,
        50.0,
        45.0,
        16.0 / 9.0,
        1.0,
        1000.0,
        10.0,
        100.0,
    );

    let eye = camera.get_eye();
    
    // With yaw=0, pitch=0, rad_yaw=0, rad_pitch=0
    // cos(0)=1, sin(0)=0
    // eye.x = target.x - distance * 1 * 0 = target.x
    // eye.y = target.y - distance * 1 * 1 = target.y - distance
    // eye.z = target.z + distance * 0 = target.z
    
    assert!((eye.0 - 100.0).abs() < 1e-4);
    assert!((eye.1 - 150.0).abs() < 1e-4);
    assert!((eye.2 - 300.0).abs() < 1e-4);
}

#[test]
fn test_camera_pitch_constraints() {
    let mut camera = OrbitCamera::new(
        (0.0, 0.0, 0.0),
        0.0,
        100.0, // should clamp to 80
        50.0,
        45.0,
        16.0 / 9.0,
        1.0,
        1000.0,
        10.0,
        100.0,
    );

    assert_eq!(camera.pitch, 80.0);

    let camera2 = OrbitCamera::new(
        (0.0, 0.0, 0.0),
        0.0,
        -100.0, // should clamp to -80
        50.0,
        45.0,
        16.0 / 9.0,
        1.0,
        1000.0,
        10.0,
        100.0,
    );
    assert_eq!(camera2.pitch, -80.0);

    camera.rotate(0.0, 50.0); // 80 + 50 = 130 -> clamp to 80
    assert_eq!(camera.pitch, 80.0);

    camera.rotate(0.0, -180.0); // 80 - 180 = -100 -> clamp to -80
    assert_eq!(camera.pitch, -80.0);
}

#[test]
fn test_camera_zoom_constraints() {
    let mut camera = OrbitCamera::new(
        (0.0, 0.0, 0.0),
        0.0,
        0.0,
        500.0, // max is 100, should clamp to 100
        45.0,
        16.0 / 9.0,
        1.0,
        1000.0,
        10.0,
        100.0,
    );

    assert_eq!(camera.distance, 100.0);

    let camera2 = OrbitCamera::new(
        (0.0, 0.0, 0.0),
        0.0,
        0.0,
        5.0, // min is 10, should clamp to 10
        45.0,
        16.0 / 9.0,
        1.0,
        1000.0,
        10.0,
        100.0,
    );
    assert_eq!(camera2.distance, 10.0);

    camera.zoom(50.0); // 100 + 50 = 150 -> clamp to 100
    assert_eq!(camera.distance, 100.0);

    camera.zoom(-150.0); // 100 - 150 = -50 -> clamp to 10
    assert_eq!(camera.distance, 10.0);
}

#[test]
fn test_camera_yaw_wrap() {
    let mut camera = OrbitCamera::new(
        (0.0, 0.0, 0.0),
        400.0, // should wrap to 40
        0.0,
        50.0,
        45.0,
        16.0 / 9.0,
        1.0,
        1000.0,
        10.0,
        100.0,
    );

    assert!((camera.yaw - 40.0).abs() < 1e-4);

    camera.rotate(-100.0, 0.0); // 40 - 100 = -60 -> wrap to 300
    assert!((camera.yaw - 300.0).abs() < 1e-4);
    
    camera.rotate(100.0, 0.0); // 300 + 100 = 400 -> wrap to 40
    assert!((camera.yaw - 40.0).abs() < 1e-4);
}

#[test]
fn test_get_view_matrix() {
    let camera = OrbitCamera::new(
        (0.0, 0.0, 0.0),
        0.0,
        0.0,
        100.0,
        45.0,
        16.0 / 9.0,
        1.0,
        1000.0,
        10.0,
        200.0,
    );

    let view_matrix = camera.get_view_matrix();

    // eye at (0, -100, 0)
    // target at (0, 0, 0)
    // up is (0, 0, 1)
    
    // Right = Forward x Up = (0, 1, 0) x (0, 0, 1) = (1, 0, 0)
    // Up' = Right x Forward = (1, 0, 0) x (0, 1, 0) = (0, 0, 1)
    
    // Expected matrix for look_at_rh with Z-up:
    // [1, 0, 0, 0]
    // [0, 0, 1, 0] // Z goes to Y (up)
    // [0, -1, 0, 0] // Y goes to -Z (forward)
    // [0, 0, -100, 1] // Translation

    assert!((view_matrix[0][0] - 1.0).abs() < 1e-4);
    assert!((view_matrix[0][1] - 0.0).abs() < 1e-4);
    assert!((view_matrix[0][2] - 0.0).abs() < 1e-4);
    
    assert!((view_matrix[1][0] - 0.0).abs() < 1e-4);
    assert!((view_matrix[1][1] - 0.0).abs() < 1e-4);
    assert!((view_matrix[1][2] - (-1.0)).abs() < 1e-4);
    
    assert!((view_matrix[2][0] - 0.0).abs() < 1e-4);
    assert!((view_matrix[2][1] - 1.0).abs() < 1e-4);
    assert!((view_matrix[2][2] - 0.0).abs() < 1e-4);
    
    assert!((view_matrix[3][0] - 0.0).abs() < 1e-4);
    assert!((view_matrix[3][1] - 0.0).abs() < 1e-4);
    assert!((view_matrix[3][2] - (-100.0)).abs() < 1e-4);
    assert!((view_matrix[3][3] - 1.0).abs() < 1e-4);
}

#[test]
fn test_get_projection_matrix() {
    let fov_degrees = 90.0_f32;
    let aspect_ratio = 1.0_f32;
    let near_z = 1.0_f32;
    let far_z = 100.0_f32;
    
    let camera = OrbitCamera::new(
        (0.0, 0.0, 0.0),
        0.0,
        0.0,
        100.0,
        fov_degrees,
        aspect_ratio,
        near_z,
        far_z,
        10.0,
        200.0,
    );

    let proj_matrix = camera.get_projection_matrix();

    // Perspective RH
    // f = 1 / tan(fov / 2) = 1 / tan(45 deg) = 1
    // m00 = f / aspect_ratio = 1 / 1 = 1
    // m11 = f = 1
    // m22 = far / (near - far) = 100 / (1 - 100) = -100 / 99 = -1.010101
    // m23 = -1
    // m32 = (near * far) / (near - far) = 100 / -99 = -1.010101
    // m33 = 0

    assert!((proj_matrix[0][0] - 1.0).abs() < 1e-4);
    assert!((proj_matrix[1][1] - 1.0).abs() < 1e-4);
    assert!((proj_matrix[2][2] - (-1.010101)).abs() < 1e-4);
    assert!((proj_matrix[2][3] - (-1.0)).abs() < 1e-4);
    assert!((proj_matrix[3][2] - (-1.010101)).abs() < 1e-4);
    assert!((proj_matrix[3][3] - 0.0).abs() < 1e-4);
}
