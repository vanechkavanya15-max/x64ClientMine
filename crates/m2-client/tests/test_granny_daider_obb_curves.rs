#[path = "../src/granny_daider_obb_curves.rs"]
mod granny_daider_obb_curves;

use granny_daider_obb_curves::{DaiderObbKey, DaiderObbCurve, generate_local_bone_matrix};
use glam::{Mat4, Quat, Vec3};
use std::f32::consts::PI;

#[test]
fn test_daider_obb_decode() {
    let key_identity = DaiderObbKey::new(0.0, 0.0, 0.0, 0.0);
    let q_id = key_identity.decode_rotation();
    assert!((q_id.w - 1.0).abs() < 1e-5);
    assert!((q_id.x - 0.0).abs() < 1e-5);
    assert!((q_id.y - 0.0).abs() < 1e-5);
    assert!((q_id.z - 0.0).abs() < 1e-5);

    let angle = PI / 2.0;
    let half_angle = angle / 2.0;
    let s = half_angle.sin();
    let key_x = DaiderObbKey::new(1.0, s, 0.0, 0.0);
    let q_x = key_x.decode_rotation();
    
    assert!((q_x.w - half_angle.cos()).abs() < 1e-5);
    assert!((q_x.x - s).abs() < 1e-5);
    assert!((q_x.y - 0.0).abs() < 1e-5);
    assert!((q_x.z - 0.0).abs() < 1e-5);
}

#[test]
fn test_curve_evaluation_slerp() {
    let q0 = Quat::IDENTITY;
    let q1 = Quat::from_rotation_x(PI / 2.0);

    let k0 = DaiderObbKey::new(0.0, q0.x, q0.y, q0.z);
    let k1 = DaiderObbKey::new(1.0, q1.x, q1.y, q1.z);
    
    let curve = DaiderObbCurve::new(vec![k0, k1]);

    let r0 = curve.evaluate_rotation(0.0);
    assert!((r0.w - q0.w).abs() < 1e-5);

    let r_mid = curve.evaluate_rotation(0.5);
    let expected_mid = q0.slerp(q1, 0.5);
    assert!((r_mid.x - expected_mid.x).abs() < 1e-5);
    assert!((r_mid.y - expected_mid.y).abs() < 1e-5);
    assert!((r_mid.z - expected_mid.z).abs() < 1e-5);
    assert!((r_mid.w - expected_mid.w).abs() < 1e-5);

    let r1 = curve.evaluate_rotation(1.0);
    assert!((r1.x - q1.x).abs() < 1e-5);
    assert!((r1.w - q1.w).abs() < 1e-5);

    let r_out_early = curve.evaluate_rotation(-0.5);
    assert!((r_out_early.x - q0.x).abs() < 1e-5);
    assert!((r_out_early.y - q0.y).abs() < 1e-5);
    assert!((r_out_early.z - q0.z).abs() < 1e-5);
    assert!((r_out_early.w - q0.w).abs() < 1e-5);

    let r_out_late = curve.evaluate_rotation(1.5);
    assert!((r_out_late.x - q1.x).abs() < 1e-5);
    assert!((r_out_late.y - q1.y).abs() < 1e-5);
    assert!((r_out_late.z - q1.z).abs() < 1e-5);
    assert!((r_out_late.w - q1.w).abs() < 1e-5);
}

#[test]
fn test_generate_local_bone_matrix() {
    let q1 = Quat::from_rotation_y(PI / 2.0);
    let k1 = DaiderObbKey::new(1.0, q1.x, q1.y, q1.z);
    let curve = DaiderObbCurve::new(vec![k1]);

    let pos = Vec3::new(1.0, 2.0, 3.0);
    let scale = Vec3::new(2.0, 2.0, 2.0);

    let mat = generate_local_bone_matrix(&curve, 1.0, pos, scale);
    let expected_mat = Mat4::from_scale_rotation_translation(scale, q1, pos);
    
    for i in 0..16 {
        assert!((mat.to_cols_array()[i] - expected_mat.to_cols_array()[i]).abs() < 1e-5);
    }
}

#[test]
fn test_slerp_shortest_path() {
    let q0 = Quat::IDENTITY;
    let mut q1 = Quat::from_rotation_z(PI * 0.75); // 135 degrees
    
    // Explicitly make q1 have a negative dot product with q0
    q1 = Quat::from_xyzw(-q1.x, -q1.y, -q1.z, -q1.w);
    
    assert!(q0.dot(q1) < 0.0);

    let k0 = DaiderObbKey::new(0.0, q0.x, q0.y, q0.z);
    let k1 = DaiderObbKey::new(1.0, q1.x, q1.y, q1.z);
    let curve = DaiderObbCurve::new(vec![k0, k1]);

    let r_mid = curve.evaluate_rotation(0.5);
    let expected_mid = q0.slerp(q1, 0.5);
    
    // We check if the rotation is physically equivalent by checking transformed basis vectors!
    for v in [Vec3::X, Vec3::Y, Vec3::Z] {
        let v_mid = r_mid * v;
        let v_expected = expected_mid * v;
        
        assert!((v_mid.x - v_expected.x).abs() < 1e-4, "x diff on basis {:?}: {} vs {}", v, v_mid.x, v_expected.x);
        assert!((v_mid.y - v_expected.y).abs() < 1e-4, "y diff on basis {:?}: {} vs {}", v, v_mid.y, v_expected.y);
        assert!((v_mid.z - v_expected.z).abs() < 1e-4, "z diff on basis {:?}: {} vs {}", v, v_mid.z, v_expected.z);
    }
}
