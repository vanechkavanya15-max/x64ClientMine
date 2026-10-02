use glam::{Mat4, Quat, Vec3};
use m2_client::granny_bone_anim_evaluator::{
    BoneTrack, GrannyAnimation, GrannyBoneAnimEvaluator, PositionKey, RotationKey,
};
use std::collections::HashMap;

#[test]
fn test_evaluate_position_lerp() {
    let mut evaluator = GrannyBoneAnimEvaluator::new();

    let track = BoneTrack {
        bone_index: 0,
        position_keys: vec![
            PositionKey {
                time: 0.0,
                position: Vec3::new(0.0, 0.0, 0.0),
            },
            PositionKey {
                time: 1.0,
                position: Vec3::new(10.0, 20.0, 30.0),
            },
        ],
        rotation_keys: vec![],
    };

    let anim = GrannyAnimation {
        name: "test_lerp".to_string(),
        duration: 1.0,
        tracks: vec![track],
    };

    evaluator.evaluate_animation(&anim, 0.5);

    let local_tf = evaluator.local_transforms.get(&0).unwrap();
    let (_, _, pos) = local_tf.to_scale_rotation_translation();

    // 0.5 lerp between 0 and 10,20,30 should be 5,10,15
    assert!((pos.x - 5.0).abs() < 1e-5);
    assert!((pos.y - 10.0).abs() < 1e-5);
    assert!((pos.z - 15.0).abs() < 1e-5);
}

#[test]
fn test_evaluate_rotation_slerp() {
    let mut evaluator = GrannyBoneAnimEvaluator::new();

    let q1 = Quat::from_axis_angle(Vec3::Y, 0.0);
    let q2 = Quat::from_axis_angle(Vec3::Y, std::f32::consts::PI);

    let track = BoneTrack {
        bone_index: 1,
        position_keys: vec![],
        rotation_keys: vec![
            RotationKey {
                time: 0.0,
                rotation: q1,
            },
            RotationKey {
                time: 2.0,
                rotation: q2,
            },
        ],
    };

    let anim = GrannyAnimation {
        name: "test_slerp".to_string(),
        duration: 2.0,
        tracks: vec![track],
    };

    // Evaluate at exactly half time
    evaluator.evaluate_animation(&anim, 1.0);

    let local_tf = evaluator.local_transforms.get(&1).unwrap();
    let (_, rot, _) = local_tf.to_scale_rotation_translation();

    // SLERP midway between 0 and PI on Y axis should be PI/2 on Y axis
    let expected_rot = Quat::from_axis_angle(Vec3::Y, std::f32::consts::PI / 2.0);

    // Note: Quat dot product gives cosine of half angle between them. If dot is near 1 or -1, they are equivalent.
    assert!(rot.angle_between(expected_rot).abs() < 1e-3 || (std::f32::consts::PI - rot.angle_between(expected_rot)).abs() < 1e-3 || rot.angle_between(-expected_rot).abs() < 1e-3);
}

#[test]
fn test_evaluate_out_of_bounds_time() {
    let mut evaluator = GrannyBoneAnimEvaluator::new();

    let track = BoneTrack {
        bone_index: 0,
        position_keys: vec![
            PositionKey {
                time: 0.5,
                position: Vec3::new(5.0, 5.0, 5.0),
            },
            PositionKey {
                time: 1.5,
                position: Vec3::new(15.0, 15.0, 15.0),
            },
        ],
        rotation_keys: vec![],
    };

    let anim = GrannyAnimation {
        name: "test_bounds".to_string(),
        duration: 2.0,
        tracks: vec![track],
    };

    // Before first key
    evaluator.evaluate_animation(&anim, 0.2);
    let local_tf_before = evaluator.local_transforms.get(&0).unwrap();
    let (_, _, pos_before) = local_tf_before.to_scale_rotation_translation();
    assert_eq!(pos_before, Vec3::new(5.0, 5.0, 5.0));

    // After last key
    evaluator.evaluate_animation(&anim, 1.8);
    let local_tf_after = evaluator.local_transforms.get(&0).unwrap();
    let (_, _, pos_after) = local_tf_after.to_scale_rotation_translation();
    assert_eq!(pos_after, Vec3::new(15.0, 15.0, 15.0));
}

#[test]
fn test_blend_animations() {
    let mut evaluator = GrannyBoneAnimEvaluator::new();

    let track_a = BoneTrack {
        bone_index: 2,
        position_keys: vec![PositionKey {
            time: 0.0,
            position: Vec3::new(0.0, 0.0, 0.0),
        }],
        rotation_keys: vec![],
    };

    let track_b = BoneTrack {
        bone_index: 2,
        position_keys: vec![PositionKey {
            time: 0.0,
            position: Vec3::new(10.0, 0.0, 0.0),
        }],
        rotation_keys: vec![],
    };

    let anim_a = GrannyAnimation {
        name: "anim_a".to_string(),
        duration: 1.0,
        tracks: vec![track_a],
    };

    let anim_b = GrannyAnimation {
        name: "anim_b".to_string(),
        duration: 1.0,
        tracks: vec![track_b],
    };

    evaluator.blend_animations(&anim_a, 0.0, &anim_b, 0.0, 0.3);

    let local_tf = evaluator.local_transforms.get(&2).unwrap();
    let (_, _, pos) = local_tf.to_scale_rotation_translation();

    // 0.3 blend between (0,0,0) and (10,0,0)
    assert!((pos.x - 3.0).abs() < 1e-5);
    assert_eq!(pos.y, 0.0);
    assert_eq!(pos.z, 0.0);
}

#[test]
fn test_calculate_global_transforms_hierarchy() {
    let mut evaluator = GrannyBoneAnimEvaluator::new();

    // Bone 0 is root, Bone 1 is child of 0
    evaluator.hierarchy.insert(1, 0);

    // Root translate by X=10
    evaluator.local_transforms.insert(
        0,
        Mat4::from_translation(Vec3::new(10.0, 0.0, 0.0)),
    );

    // Child translate by Y=5 locally
    evaluator.local_transforms.insert(
        1,
        Mat4::from_translation(Vec3::new(0.0, 5.0, 0.0)),
    );

    evaluator.calculate_global_transforms();

    let global_0 = evaluator.global_transforms.get(&0).unwrap();
    let global_1 = evaluator.global_transforms.get(&1).unwrap();

    let (_, _, pos_0) = global_0.to_scale_rotation_translation();
    let (_, _, pos_1) = global_1.to_scale_rotation_translation();

    // Root should be at (10, 0, 0)
    assert_eq!(pos_0, Vec3::new(10.0, 0.0, 0.0));

    // Child should be at Root + Local = (10, 5, 0)
    assert_eq!(pos_1, Vec3::new(10.0, 5.0, 0.0));
}

#[test]
fn test_compute_skin_matrices() {
    let mut evaluator = GrannyBoneAnimEvaluator::new();

    // Setup global transform
    let global_tf = Mat4::from_translation(Vec3::new(10.0, 20.0, 30.0));
    evaluator.global_transforms.insert(5, global_tf);

    // Setup inverse bind pose
    // Let's say bind pose was translation (10, 20, 30). Then inv bind is translation (-10, -20, -30)
    let inv_bind = Mat4::from_translation(Vec3::new(-10.0, -20.0, -30.0));
    evaluator.inv_bind_poses.insert(5, inv_bind);

    evaluator.compute_skin_matrices();

    let skin_mat = evaluator.skin_matrices.get(&5).unwrap();
    let (_, _, pos) = skin_mat.to_scale_rotation_translation();

    // Since global tf is same as bind pose, skin matrix should be identity
    assert!((pos.x - 0.0).abs() < 1e-5);
    assert!((pos.y - 0.0).abs() < 1e-5);
    assert!((pos.z - 0.0).abs() < 1e-5);
}
