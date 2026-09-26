use m2_client::camera_orbit::CameraOrbit;
use std::f32::consts::PI;

const EPSILON: f32 = 0.001;

fn assert_vec3_approx_eq(a: [f32; 3], b: [f32; 3]) {
    assert!((a[0] - b[0]).abs() < EPSILON, "X mismatch: {} != {}", a[0], b[0]);
    assert!((a[1] - b[1]).abs() < EPSILON, "Y mismatch: {} != {}", a[1], b[1]);
    assert!((a[2] - b[2]).abs() < EPSILON, "Z mismatch: {} != {}", a[2], b[2]);
}

#[test]
fn test_eye_position_yaw_rotation() {
    let target = [0.0, 0.0, 0.0];
    let distance = 1000.0;
    
    // Pitch 0 to simplify testing yaw effect on XY plane (pitch clamp will adjust this to 10 degrees)
    // So pitch = 10 degrees
    let pitch = 10.0;
    let rad_pitch = pitch * PI / 180.0;
    let expected_z = distance * rad_pitch.sin();
    
    // Yaw 0 degrees (pointing positive Y)
    let cam1 = CameraOrbit::new(target, 0.0, 0.0, distance);
    let eye1 = cam1.get_eye_position();
    // expected eye_y: 0 - 1000 * cos(10) * cos(0)
    let expected_y1 = -distance * rad_pitch.cos();
    assert_vec3_approx_eq(eye1, [0.0, expected_y1, expected_z]);
    
    // Yaw 90 degrees (pointing positive X)
    let cam2 = CameraOrbit::new(target, 0.0, 90.0, distance);
    let eye2 = cam2.get_eye_position();
    let expected_x2 = -distance * rad_pitch.cos();
    assert_vec3_approx_eq(eye2, [expected_x2, 0.0, expected_z]);

    // Yaw 180 degrees (pointing negative Y)
    let cam3 = CameraOrbit::new(target, 0.0, 180.0, distance);
    let eye3 = cam3.get_eye_position();
    assert_vec3_approx_eq(eye3, [0.0, -expected_y1, expected_z]);
}

#[test]
fn test_pitch_constraints() {
    let target = [100.0, 200.0, 300.0];
    
    let mut cam = CameraOrbit::new(target, 5.0, 0.0, 1000.0);
    assert_eq!(cam.pitch_deg, 10.0, "Pitch should clamp to 10.0 min");
    
    cam.set_pitch(-100.0);
    assert_eq!(cam.pitch_deg, 10.0, "Pitch should clamp to 10.0 min");
    
    cam.set_pitch(15.0);
    assert_eq!(cam.pitch_deg, 15.0, "Pitch inside limits should not change");
    
    cam.set_pitch(90.0);
    assert_eq!(cam.pitch_deg, 80.0, "Pitch should clamp to 80.0 max");
}

#[test]
fn test_distance_constraints() {
    let target = [0.0, 0.0, 0.0];
    
    let mut cam = CameraOrbit::new(target, 30.0, 0.0, 100.0);
    assert_eq!(cam.distance, 500.0, "Distance should clamp to 500.0 min");
    
    cam.set_distance(1500.0);
    assert_eq!(cam.distance, 1500.0, "Distance inside limits should not change");
    
    cam.set_distance(5000.0);
    assert_eq!(cam.distance, 3500.0, "Distance should clamp to 3500.0 max");
}

#[test]
fn test_target_point_behavior() {
    let mut cam = CameraOrbit::new([0.0, 0.0, 0.0], 30.0, 45.0, 1000.0);
    let eye_initial = cam.get_eye_position();
    
    cam.set_target([100.0, 200.0, 300.0]);
    let eye_new = cam.get_eye_position();
    
    assert_vec3_approx_eq(eye_new, [
        eye_initial[0] + 100.0,
        eye_initial[1] + 200.0,
        eye_initial[2] + 300.0
    ]);
}

#[test]
fn test_edge_cases_and_lookat() {
    let target = [0.0, 0.0, 0.0];
    // Yaw = 0, Pitch = 45
    let mut cam = CameraOrbit::new(target, 45.0, 0.0, 1000.0);
    
    let mat = cam.get_view_matrix();
    // Validate look at doesn't have NaN or infinite values
    for val in mat.iter() {
        assert!(!val.is_nan(), "View matrix should not contain NaN");
        assert!(!val.is_infinite(), "View matrix should not contain infinite values");
    }

    // Since yaw is 0, eye is at -Y and +Z relative to target
    // The camera looks AT target (which is at origin), so looking in +Y and -Z
    // The Z-axis of the camera (look vector mapped to -Z in D3D, but zaxis in lookat points FROM target TO eye if right-handed or vice versa)
    // Actually standard lookAt returns a matrix. The specific values aren't as important as them being valid and consistent with math
    let eye = cam.get_eye_position();
    // Validate eye
    let d = 1000.0;
    let angle = 45.0_f32.to_radians();
    assert_vec3_approx_eq(eye, [0.0, -d * angle.cos(), d * angle.sin()]);
}
