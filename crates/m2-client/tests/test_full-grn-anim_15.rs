



use glam::Mat4;
use m2_client::granny_animation_evaluator::AnimationTrack;
use m2_client::granny_engine_validator::{GrannyEngineValidator, ValidationError};

#[test]
fn test_valid_animation_track() {
    let track = AnimationTrack::new(
        2.0,
        30.0,
        false,
        vec![
            vec![Mat4::IDENTITY, Mat4::IDENTITY],
            vec![Mat4::IDENTITY, Mat4::IDENTITY],
        ],
    );
    assert_eq!(GrannyEngineValidator::validate_animation_track(&track), Ok(()));
}

#[test]
fn test_invalid_duration_and_fps() {
    let track_invalid_duration = AnimationTrack::new(
        -1.0,
        30.0,
        false,
        vec![vec![Mat4::IDENTITY]],
    );
    assert_eq!(GrannyEngineValidator::validate_animation_track(&track_invalid_duration), Err(ValidationError::InvalidDuration));

    let track_invalid_fps = AnimationTrack::new(
        2.0,
        0.0,
        false,
        vec![vec![Mat4::IDENTITY]],
    );
    assert_eq!(GrannyEngineValidator::validate_animation_track(&track_invalid_fps), Err(ValidationError::InvalidFps));
}

#[test]
fn test_mismatched_bone_counts() {
    let track = AnimationTrack::new(
        2.0,
        30.0,
        false,
        vec![
            vec![Mat4::IDENTITY, Mat4::IDENTITY],
            vec![Mat4::IDENTITY],
        ],
    );
    assert_eq!(GrannyEngineValidator::validate_animation_track(&track), Err(ValidationError::MismatchedBoneCounts));
}

#[test]
fn test_invalid_transform_matrix() {
    let mut invalid_mat = Mat4::IDENTITY;
    invalid_mat.x_axis.x = f32::NAN;
    
    let track = AnimationTrack::new(
        2.0,
        30.0,
        false,
        vec![vec![invalid_mat]],
    );
    assert_eq!(GrannyEngineValidator::validate_animation_track(&track), Err(ValidationError::InvalidTransformMatrix));
}

#[test]
fn test_buffer_overflow_in_transform_buffer() {
    let bones = vec![Mat4::IDENTITY, Mat4::IDENTITY, Mat4::IDENTITY];
    
    // Valid case: exactly max_bones
    assert_eq!(GrannyEngineValidator::validate_transform_buffer(&bones, 3), Ok(()));
    
    // Overflow case: more bones than max_bones
    assert_eq!(GrannyEngineValidator::validate_transform_buffer(&bones, 2), Err(ValidationError::BufferOverflow));
}

#[test]
fn test_empty_frames() {
    let track = AnimationTrack::new(
        2.0,
        30.0,
        false,
        vec![],
    );
    assert_eq!(GrannyEngineValidator::validate_animation_track(&track), Err(ValidationError::EmptyFrames));
}
