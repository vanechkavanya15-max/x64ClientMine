use glam::{Mat4, Quat, Vec3};
use m2_client::granny_animation_evaluator::{
    calculate_loop_time, evaluate_animation_at_time, AnimationTrack, TrackSample,
};

#[test]
fn test_calculate_loop_time_no_loop() {
    let duration = 2.0;
    
    // Normal time within bounds
    assert_eq!(calculate_loop_time(1.0, duration, false), 1.0);
    
    // Clamped below zero
    assert_eq!(calculate_loop_time(-0.5, duration, false), 0.0);
    
    // Clamped above duration
    assert_eq!(calculate_loop_time(2.5, duration, false), 2.0);
}

#[test]
fn test_calculate_loop_time_with_loop() {
    let duration = 2.0;
    
    // Normal time within bounds
    assert_eq!(calculate_loop_time(1.0, duration, true), 1.0);
    
    // Wrapped forward
    assert_eq!(calculate_loop_time(2.5, duration, true), 0.5);
    
    // Wrapped multiple times forward
    assert_eq!(calculate_loop_time(5.0, duration, true), 1.0);
    
    // Wrapped backward
    assert_eq!(calculate_loop_time(-0.5, duration, true), 1.5);
    
    // Wrapped multiple times backward
    assert_eq!(calculate_loop_time(-2.5, duration, true), 1.5);
}

#[test]
fn test_calculate_loop_time_zero_duration() {
    assert_eq!(calculate_loop_time(1.0, 0.0, true), 0.0);
    assert_eq!(calculate_loop_time(1.0, -1.0, false), 0.0);
}

#[test]
fn test_evaluate_animation_at_time_empty() {
    let track = AnimationTrack::new(1.0, 30.0, false, vec![]);
    
    let sample = evaluate_animation_at_time(&track, 0.5);
    assert_eq!(sample.time, 0.5);
    assert!(sample.bone_transforms.is_empty());
}

#[test]
fn test_evaluate_animation_at_time_single_frame() {
    let m0 = Mat4::from_translation(Vec3::new(1.0, 2.0, 3.0));
    let frames = vec![vec![m0]];
    let track = AnimationTrack::new(1.0, 30.0, false, frames.clone());
    
    let sample = evaluate_animation_at_time(&track, 0.5);
    assert_eq!(sample.time, 0.5);
    assert_eq!(sample.bone_transforms.len(), 1);
    assert_eq!(sample.bone_transforms[0], m0);
}

#[test]
fn test_evaluate_animation_at_time_exact_frame() {
    let m0 = Mat4::from_translation(Vec3::new(1.0, 0.0, 0.0));
    let m1 = Mat4::from_translation(Vec3::new(2.0, 0.0, 0.0));
    let frames = vec![vec![m0], vec![m1]];
    
    // 2 frames at 1 fps means frame0 is at t=0, frame1 is at t=1
    let track = AnimationTrack::new(2.0, 1.0, false, frames);
    
    // Exactly at frame 0
    let sample0 = evaluate_animation_at_time(&track, 0.0);
    assert_eq!(sample0.bone_transforms[0], m0);
    
    // Exactly at frame 1
    let sample1 = evaluate_animation_at_time(&track, 1.0);
    assert_eq!(sample1.bone_transforms[0], m1);
}

#[test]
fn test_evaluate_animation_at_time_interpolation_no_loop() {
    let m0 = Mat4::from_translation(Vec3::new(0.0, 0.0, 0.0));
    let m1 = Mat4::from_translation(Vec3::new(10.0, 0.0, 0.0));
    let frames = vec![vec![m0], vec![m1]];
    
    // 2 frames at 1 fps means frame0 is at t=0, frame1 is at t=1
    let track = AnimationTrack::new(2.0, 1.0, false, frames);
    
    // At t=0.5, it should be exactly halfway between m0 and m1
    let sample = evaluate_animation_at_time(&track, 0.5);
    
    let expected = Mat4::from_translation(Vec3::new(5.0, 0.0, 0.0));
    
    // Due to floating point precision, we check components rather than exact equality
    let (_, _, t_interp) = sample.bone_transforms[0].to_scale_rotation_translation();
    assert!((t_interp.x - 5.0).abs() < 1e-5);
    assert_eq!(t_interp.y, 0.0);
    assert_eq!(t_interp.z, 0.0);
}

#[test]
fn test_evaluate_animation_at_time_interpolation_loop() {
    let m0 = Mat4::from_translation(Vec3::new(0.0, 0.0, 0.0));
    let m1 = Mat4::from_translation(Vec3::new(10.0, 0.0, 0.0));
    let frames = vec![vec![m0], vec![m1]];
    
    // 2 frames at 1 fps means frame0 is at t=0, frame1 is at t=1
    let track = AnimationTrack::new(2.0, 1.0, true, frames);
    
    // With loop enabled, at t=1.5, we should interpolate between m1 (t=1) and m0 (t=2 which loops to 0)
    let sample = evaluate_animation_at_time(&track, 1.5);
    
    // It should be halfway between m1 and m0
    let (_, _, t_interp) = sample.bone_transforms[0].to_scale_rotation_translation();
    assert!((t_interp.x - 5.0).abs() < 1e-5);
    assert_eq!(t_interp.y, 0.0);
    assert_eq!(t_interp.z, 0.0);
}

#[test]
fn test_evaluate_animation_at_time_complex_transforms() {
    let q0 = Quat::from_rotation_y(0.0);
    let q1 = Quat::from_rotation_y(std::f32::consts::PI / 2.0);
    
    let m0 = Mat4::from_scale_rotation_translation(
        Vec3::new(1.0, 1.0, 1.0),
        q0,
        Vec3::new(0.0, 0.0, 0.0)
    );
    let m1 = Mat4::from_scale_rotation_translation(
        Vec3::new(2.0, 2.0, 2.0),
        q1,
        Vec3::new(10.0, 0.0, 0.0)
    );
    
    let frames = vec![vec![m0], vec![m1]];
    let track = AnimationTrack::new(2.0, 1.0, false, frames);
    
    let sample = evaluate_animation_at_time(&track, 0.5);
    
    let (s, q, t) = sample.bone_transforms[0].to_scale_rotation_translation();
    
    // Scale should be 1.5
    assert!((s.x - 1.5).abs() < 1e-5);
    
    // Translation should be 5.0 on x
    assert!((t.x - 5.0).abs() < 1e-5);
    
    // Rotation should be interpolated halfway
    let expected_q = Quat::from_rotation_y(std::f32::consts::PI / 4.0);
    assert!((q.x - expected_q.x).abs() < 1e-5);
    assert!((q.y - expected_q.y).abs() < 1e-5);
    assert!((q.z - expected_q.z).abs() < 1e-5);
    assert!((q.w - expected_q.w).abs() < 1e-5);
}
