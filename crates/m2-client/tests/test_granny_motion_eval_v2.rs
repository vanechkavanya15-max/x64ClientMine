use m2_client::granny_motion_eval_v2::*;
use glam::{Quat, Vec3};
use std::collections::HashMap;
use std::sync::Mutex;

static TEST_LOCK: Mutex<()> = Mutex::new(());

fn approx_eq(a: f32, b: f32) -> bool {
    (a - b).abs() < 1e-5
}

#[test]
fn test_motion_eval_bone_not_found() {
    let _lock = TEST_LOCK.lock().unwrap();
    clear_animations();
    let res = m2_motion_eval_bone(999, 1, 0.0);
    assert_eq!(res, (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0));
}

#[test]
fn test_motion_eval_bone_empty_keys() {
    let _lock = TEST_LOCK.lock().unwrap();
    let bone = BoneAnimation {
        translation_keys: vec![],
        rotation_keys: vec![],
    };
    let mut bones = HashMap::new();
    bones.insert(101, bone);
    let anim = AnimationCurve { anim_id: 101, bones };
    register_animation(anim);

    let res = m2_motion_eval_bone(101, 101, 1.0);
    assert_eq!(res, (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0));
}

#[test]
fn test_motion_eval_bone_single_key() {
    let _lock = TEST_LOCK.lock().unwrap();
    let bone = BoneAnimation {
        translation_keys: vec![TranslationKey { time: 1.0, val: Vec3::new(10.0, 20.0, 30.0) }],
        rotation_keys: vec![RotationKey { time: 1.0, val: Quat::from_xyzw(0.1, 0.2, 0.3, 0.9).normalize() }],
    };
    let mut bones = HashMap::new();
    bones.insert(102, bone);
    let anim = AnimationCurve { anim_id: 102, bones };
    register_animation(anim);

    let res = m2_motion_eval_bone(102, 102, 0.5); // Before key
    let q = Quat::from_xyzw(0.1, 0.2, 0.3, 0.9).normalize();
    assert!(approx_eq(res.0, 10.0));
    assert!(approx_eq(res.1, 20.0));
    assert!(approx_eq(res.2, 30.0));
    assert!(approx_eq(res.3, q.x));
    assert!(approx_eq(res.4, q.y));
    assert!(approx_eq(res.5, q.z));
    assert!(approx_eq(res.6, q.w));

    let res = m2_motion_eval_bone(102, 102, 1.5); // After key
    assert!(approx_eq(res.0, 10.0));
}

#[test]
fn test_motion_eval_bone_interpolation() {
    let _lock = TEST_LOCK.lock().unwrap();
    let bone = BoneAnimation {
        translation_keys: vec![
            TranslationKey { time: 0.0, val: Vec3::new(0.0, 0.0, 0.0) },
            TranslationKey { time: 1.0, val: Vec3::new(10.0, 20.0, 30.0) },
        ],
        rotation_keys: vec![
            RotationKey { time: 0.0, val: Quat::IDENTITY },
            RotationKey { time: 1.0, val: Quat::from_axis_angle(Vec3::Z, std::f32::consts::PI / 2.0) },
        ],
    };
    let mut bones = HashMap::new();
    bones.insert(103, bone);
    let anim = AnimationCurve { anim_id: 103, bones };
    register_animation(anim);

    // Halfway
    let res = m2_motion_eval_bone(103, 103, 0.5);
    assert!(approx_eq(res.0, 5.0));
    assert!(approx_eq(res.1, 10.0));
    assert!(approx_eq(res.2, 15.0));
    
    let expected_q = Quat::IDENTITY.slerp(Quat::from_axis_angle(Vec3::Z, std::f32::consts::PI / 2.0), 0.5);
    assert!(approx_eq(res.3, expected_q.x));
    assert!(approx_eq(res.4, expected_q.y));
    assert!(approx_eq(res.5, expected_q.z));
    assert!(approx_eq(res.6, expected_q.w));

    // Beyond last key
    let res2 = m2_motion_eval_bone(103, 103, 2.0);
    assert!(approx_eq(res2.0, 10.0));
    assert!(approx_eq(res2.1, 20.0));
    assert!(approx_eq(res2.2, 30.0));
    
    // Before first key
    let res3 = m2_motion_eval_bone(103, 103, -1.0);
    assert!(approx_eq(res3.0, 0.0));
    assert!(approx_eq(res3.1, 0.0));
    assert!(approx_eq(res3.2, 0.0));
}

#[test]
fn test_motion_eval_bone_multiple_segments() {
    let _lock = TEST_LOCK.lock().unwrap();
    let bone = BoneAnimation {
        translation_keys: vec![
            TranslationKey { time: 0.0, val: Vec3::new(0.0, 0.0, 0.0) },
            TranslationKey { time: 1.0, val: Vec3::new(10.0, 0.0, 0.0) },
            TranslationKey { time: 2.0, val: Vec3::new(10.0, 10.0, 0.0) },
        ],
        rotation_keys: vec![],
    };
    let mut bones = HashMap::new();
    bones.insert(104, bone);
    let anim = AnimationCurve { anim_id: 104, bones };
    register_animation(anim);

    // Mid first segment
    let res = m2_motion_eval_bone(104, 104, 0.5);
    assert!(approx_eq(res.0, 5.0));
    assert!(approx_eq(res.1, 0.0));
    
    // Exact on key
    let res = m2_motion_eval_bone(104, 104, 1.0);
    assert!(approx_eq(res.0, 10.0));
    assert!(approx_eq(res.1, 0.0));

    // Mid second segment
    let res = m2_motion_eval_bone(104, 104, 1.5);
    assert!(approx_eq(res.0, 10.0));
    assert!(approx_eq(res.1, 5.0));
}
