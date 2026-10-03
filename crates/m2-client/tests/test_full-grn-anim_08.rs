#[path = "../src/granny_motion_crossfader.rs"]
mod granny_motion_crossfader;

use glam::{Vec3, Quat};
use granny_motion_crossfader::{BoneTransform, MotionCrossfader};

fn approx_eq(a: f32, b: f32) -> bool {
    (a - b).abs() < 1e-5
}

fn vec3_approx_eq(a: Vec3, b: Vec3) -> bool {
    approx_eq(a.x, b.x) && approx_eq(a.y, b.y) && approx_eq(a.z, b.z)
}

fn quat_approx_eq(a: Quat, b: Quat) -> bool {
    let dot = a.dot(b).abs();
    (1.0 - dot) < 1e-5
}

#[test]
fn test_bone_transform_blend_halfway() {
    let source = BoneTransform::new(
        Vec3::new(0.0, 0.0, 0.0),
        Quat::from_axis_angle(Vec3::Y, 0.0),
        Vec3::new(1.0, 1.0, 1.0),
    );

    let target = BoneTransform::new(
        Vec3::new(10.0, 20.0, 30.0),
        Quat::from_axis_angle(Vec3::Y, std::f32::consts::PI / 2.0),
        Vec3::new(2.0, 2.0, 2.0),
    );

    let blended = source.blend(&target, 0.5);

    assert!(vec3_approx_eq(blended.position, Vec3::new(5.0, 10.0, 15.0)));
    assert!(vec3_approx_eq(blended.scale, Vec3::new(1.5, 1.5, 1.5)));

    let expected_rotation = Quat::from_axis_angle(Vec3::Y, std::f32::consts::PI / 4.0);
    assert!(quat_approx_eq(blended.rotation, expected_rotation));
}

#[test]
fn test_bone_transform_blend_clamping() {
    let source = BoneTransform::new(
        Vec3::ZERO,
        Quat::IDENTITY,
        Vec3::ONE,
    );

    let target = BoneTransform::new(
        Vec3::new(10.0, 0.0, 0.0),
        Quat::IDENTITY,
        Vec3::ONE,
    );

    let over_blend = source.blend(&target, 1.5);
    assert!(vec3_approx_eq(over_blend.position, Vec3::new(10.0, 0.0, 0.0)));

    let under_blend = source.blend(&target, -0.5);
    assert!(vec3_approx_eq(under_blend.position, Vec3::ZERO));
}

#[test]
fn test_motion_crossfader_zero_duration() {
    let source = vec![BoneTransform::new(Vec3::ZERO, Quat::IDENTITY, Vec3::ONE)];
    let target = vec![BoneTransform::new(Vec3::new(10.0, 0.0, 0.0), Quat::IDENTITY, Vec3::ONE)];

    let mut crossfader = MotionCrossfader::new(0.0, source, target.clone());
    
    assert!(crossfader.is_finished());
    assert_eq!(crossfader.get_blend_weight(), 1.0);
    
    let blended = crossfader.get_blended_pose();
    assert!(vec3_approx_eq(blended[0].position, target[0].position));
}

#[test]
fn test_motion_crossfader_progression() {
    let source = vec![BoneTransform::new(Vec3::ZERO, Quat::IDENTITY, Vec3::ONE)];
    let target = vec![BoneTransform::new(Vec3::new(10.0, 0.0, 0.0), Quat::IDENTITY, Vec3::ONE)];

    let mut crossfader = MotionCrossfader::new(2.0, source, target);
    
    assert_eq!(crossfader.get_blend_weight(), 0.0);
    assert!(!crossfader.is_finished());

    crossfader.update(1.0);
    assert_eq!(crossfader.get_blend_weight(), 0.5);
    assert!(!crossfader.is_finished());
    
    let blended = crossfader.get_blended_pose();
    assert!(vec3_approx_eq(blended[0].position, Vec3::new(5.0, 0.0, 0.0)));

    crossfader.update(1.5);
    assert_eq!(crossfader.get_blend_weight(), 1.0);
    assert!(crossfader.is_finished());
    
    let blended = crossfader.get_blended_pose();
    assert!(vec3_approx_eq(blended[0].position, Vec3::new(10.0, 0.0, 0.0)));
}

#[test]
#[should_panic(expected = "Source and target poses must have the same number of bones")]
fn test_motion_crossfader_mismatched_poses() {
    let source = vec![BoneTransform::new(Vec3::ZERO, Quat::IDENTITY, Vec3::ONE)];
    let target = vec![
        BoneTransform::new(Vec3::ZERO, Quat::IDENTITY, Vec3::ONE),
        BoneTransform::new(Vec3::ZERO, Quat::IDENTITY, Vec3::ONE),
    ];

    let _crossfader = MotionCrossfader::new(1.0, source, target);
}

#[test]
fn test_motion_crossfader_multi_bone_array_blending() {
    let source = vec![
        BoneTransform::new(Vec3::new(0.0, 0.0, 0.0), Quat::IDENTITY, Vec3::ONE),
        BoneTransform::new(Vec3::new(0.0, 0.0, 0.0), Quat::IDENTITY, Vec3::ONE),
    ];
    let target = vec![
        BoneTransform::new(Vec3::new(10.0, 0.0, 0.0), Quat::IDENTITY, Vec3::ONE),
        BoneTransform::new(Vec3::new(0.0, 20.0, 0.0), Quat::IDENTITY, Vec3::ONE),
    ];

    let mut crossfader = MotionCrossfader::new(1.0, source, target);
    crossfader.update(0.5);
    
    let blended = crossfader.get_blended_pose();
    assert_eq!(blended.len(), 2);
    assert!(vec3_approx_eq(blended[0].position, Vec3::new(5.0, 0.0, 0.0)));
    assert!(vec3_approx_eq(blended[1].position, Vec3::new(0.0, 10.0, 0.0)));
}
