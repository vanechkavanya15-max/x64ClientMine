use m2_client_render::camera_3d::Camera3d;
use glam::Vec3;

#[test]
fn test_integration_camera_3d() {
    let instance = Camera3d::new();
    assert!(instance.is_ready());
}

#[test]
fn test_camera_integration_matrices() {
    let mut cam = Camera3d::new();
    assert_eq!(cam.distance, 10.0);
    assert_eq!(cam.target, Vec3::ZERO);
    assert_eq!(cam.yaw, 0.0);
    assert_eq!(cam.pitch, 0.0);

    cam.set_distance(5.0);
    assert_eq!(cam.distance, 5.0);

    cam.set_target(Vec3::new(1.0, 2.0, 3.0));
    assert_eq!(cam.target, Vec3::new(1.0, 2.0, 3.0));

    cam.set_rotation(std::f32::consts::PI / 2.0, std::f32::consts::PI / 4.0);
    assert!((cam.yaw - std::f32::consts::PI / 2.0).abs() < 1e-5);
    assert!((cam.pitch - std::f32::consts::PI / 4.0).abs() < 1e-5);

    // Sprawdzamy, czy macierze zmienily sie w zaleznosci od wejsc (tzn. ze update matrix zadzialalo i zaktualizowalo macierze)
    // view matrix sie zmienia kiedy zmienia sie offset i cel kamery.
    let mat = cam.view_matrix;
    assert!(mat != glam::Mat4::IDENTITY);
}

#[test]
fn test_camera_clamp_pitch() {
    let mut cam = Camera3d::new();
    let limit = std::f32::consts::FRAC_PI_2 - 0.01;
    cam.set_rotation(0.0, 5.0);
    assert!((cam.pitch - limit).abs() < 1e-5);
    
    cam.set_rotation(0.0, -5.0);
    assert!((cam.pitch - (-limit)).abs() < 1e-5);
}

#[test]
fn test_camera_clamp_distance() {
    let mut cam = Camera3d::new();
    cam.set_distance(-5.0);
    assert_eq!(cam.distance, 0.1);
}
