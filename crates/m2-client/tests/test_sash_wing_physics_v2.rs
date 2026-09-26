use m2_client::sash_wing_physics_v2::*;
use glam::Mat4;
use std::sync::{Mutex, OnceLock};

static TEST_MUTEX: OnceLock<Mutex<()>> = OnceLock::new();

fn get_test_mutex() -> &'static Mutex<()> {
    TEST_MUTEX.get_or_init(|| Mutex::new(()))
}

#[test]
fn test_sash_initial_transform() {
    let _lock = get_test_mutex().lock().unwrap();
    test_reset_sash_state();

    let mut out_mat = [0.0f32; 16];
    let result = m2_sash_get_transform(out_mat.as_mut_ptr());
    
    assert!(result, "Expected getting transform to succeed");
    
    // Initial state should be identity matrix (no bend)
    let mat = Mat4::from_cols_array(&out_mat);
    assert_eq!(mat, Mat4::IDENTITY);
}

#[test]
fn test_sash_forward_movement_bends_pitch() {
    let _lock = get_test_mutex().lock().unwrap();
    test_reset_sash_state();

    // Start at origin, yaw = 0.0 (facing +Y forward)
    m2_sash_update(0.0, 0.0, 0.0, 0.0, 16);

    // Move forward (+Y)
    m2_sash_update(0.0, 10.0, 0.0, 0.0, 16);

    let mut out_mat = [0.0f32; 16];
    assert!(m2_sash_get_transform(out_mat.as_mut_ptr()));

    let mat = Mat4::from_cols_array(&out_mat);
    assert!(mat != Mat4::IDENTITY, "Matrix should not be identity after movement");
    
    // Extracted matrix should represent pitch bend (due to forward acceleration)
    // We expect it to bend negative pitch
    let (scale, rot, trans) = mat.to_scale_rotation_translation();
    let (pitch, roll, yaw) = rot.to_euler(glam::EulerRot::XYZ);
    
    assert!(pitch < 0.0, "Expected backward bend (negative pitch) when accelerating forward");
    assert!(roll.abs() < 0.001, "Roll should be zero when moving straight forward");
    assert!(yaw.abs() < 0.001, "Yaw should be zero");
}

#[test]
fn test_sash_side_movement_bends_roll() {
    let _lock = get_test_mutex().lock().unwrap();
    test_reset_sash_state();

    // Start at origin
    m2_sash_update(0.0, 0.0, 0.0, 0.0, 16);

    // Move sideways (+X)
    m2_sash_update(10.0, 0.0, 0.0, 0.0, 16);

    let mut out_mat = [0.0f32; 16];
    assert!(m2_sash_get_transform(out_mat.as_mut_ptr()));

    let mat = Mat4::from_cols_array(&out_mat);
    let (scale, rot, trans) = mat.to_scale_rotation_translation();
    let (pitch, roll, yaw) = rot.to_euler(glam::EulerRot::XYZ);
    
    assert!(pitch.abs() < 0.001, "Pitch should be zero when moving straight sideways");
    assert!(roll < 0.0, "Expected roll bend when moving sideways");
}

#[test]
fn test_sash_constant_velocity_returns_to_rest() {
    let _lock = get_test_mutex().lock().unwrap();
    test_reset_sash_state();

    // Accelerate
    m2_sash_update(0.0, 0.0, 0.0, 0.0, 16);
    m2_sash_update(0.0, 10.0, 0.0, 0.0, 16);
    
    // Constant velocity
    for i in 2..20 {
        m2_sash_update(0.0, i as f32 * 10.0, 0.0, 0.0, 16);
    }

    let mut out_mat = [0.0f32; 16];
    assert!(m2_sash_get_transform(out_mat.as_mut_ptr()));

    let mat = Mat4::from_cols_array(&out_mat);
    let (_, rot, _) = mat.to_scale_rotation_translation();
    let (pitch, _, _) = rot.to_euler(glam::EulerRot::XYZ);
    
    // Bend should start returning to 0 when acceleration is 0
    assert!(pitch.abs() < 0.1, "Pitch should eventually return close to 0 under constant velocity, current pitch: {}", pitch);
}

#[test]
fn test_sash_update_zero_delta_time() {
    let _lock = get_test_mutex().lock().unwrap();
    test_reset_sash_state();
    
    m2_sash_update(0.0, 0.0, 0.0, 0.0, 16);
    
    // Update with 0 delta time
    let result = m2_sash_update(0.0, 10.0, 0.0, 0.0, 0);
    assert!(result, "Should handle 0 delta time gracefully");
    
    let mut out_mat = [0.0f32; 16];
    assert!(m2_sash_get_transform(out_mat.as_mut_ptr()));

    // State shouldn't change
    let mat = Mat4::from_cols_array(&out_mat);
    assert_eq!(mat, Mat4::IDENTITY);
}

#[test]
fn test_sash_get_transform_null_pointer() {
    let _lock = get_test_mutex().lock().unwrap();
    
    let result = m2_sash_get_transform(std::ptr::null_mut());
    assert!(!result, "Should return false on null pointer");
}

#[test]
fn test_sash_yaw_orientation() {
    let _lock = get_test_mutex().lock().unwrap();
    test_reset_sash_state();
    
    // If facing right (yaw = 90 deg), and moving right (+X), it is forward movement in local space
    // So pitch should bend backwards (negative), not roll.
    m2_sash_update(0.0, 0.0, 0.0, 90.0, 16);
    m2_sash_update(10.0, 0.0, 0.0, 90.0, 16);

    let mut out_mat = [0.0f32; 16];
    assert!(m2_sash_get_transform(out_mat.as_mut_ptr()));

    let mat = Mat4::from_cols_array(&out_mat);
    let (_, rot, _) = mat.to_scale_rotation_translation();
    let (pitch, roll, _) = rot.to_euler(glam::EulerRot::XYZ);
    
    assert!(pitch < 0.0, "Expected pitch bend since movement is local-forward");
    assert!(roll.abs() < 0.001, "Expected no roll bend since movement is local-forward");
}
