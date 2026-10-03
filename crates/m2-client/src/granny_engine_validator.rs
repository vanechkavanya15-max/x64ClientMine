use glam::Mat4;
use crate::granny_animation_evaluator::AnimationTrack;

#[derive(Debug, PartialEq)]
pub enum ValidationError {
    InvalidDuration,
    InvalidFps,
    EmptyFrames,
    MismatchedBoneCounts,
    InvalidTransformMatrix,
    BufferOverflow,
}

pub struct GrannyEngineValidator;

impl GrannyEngineValidator {
    pub fn validate_animation_track(track: &AnimationTrack) -> Result<(), ValidationError> {
        if track.duration_seconds <= 0.0 || track.duration_seconds.is_nan() {
            return Err(ValidationError::InvalidDuration);
        }

        if track.frames_per_second <= 0.0 || track.frames_per_second.is_nan() {
            return Err(ValidationError::InvalidFps);
        }

        if track.frames.is_empty() {
            return Err(ValidationError::EmptyFrames);
        }

        let first_frame_bone_count = track.frames[0].len();
        for frame in &track.frames {
            if frame.len() != first_frame_bone_count {
                return Err(ValidationError::MismatchedBoneCounts);
            }
            
            for bone in frame {
                if !Self::is_valid_mat4(bone) {
                    return Err(ValidationError::InvalidTransformMatrix);
                }
            }
        }

        Ok(())
    }

    pub fn validate_transform_buffer(bone_transforms: &[Mat4], max_bones: usize) -> Result<(), ValidationError> {
        if bone_transforms.len() > max_bones {
            return Err(ValidationError::BufferOverflow);
        }

        for bone in bone_transforms {
            if !Self::is_valid_mat4(bone) {
                return Err(ValidationError::InvalidTransformMatrix);
            }
        }

        Ok(())
    }

    fn is_valid_mat4(mat: &Mat4) -> bool {
        let cols = mat.to_cols_array();
        for val in cols {
            if val.is_nan() || val.is_infinite() {
                return false;
            }
        }
        true
    }
}
