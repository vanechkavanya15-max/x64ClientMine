use glam::Mat4;

#[derive(Debug, Clone)]
pub struct AnimationTrack {
    pub duration_seconds: f32,
    pub frames_per_second: f32,
    pub is_looping: bool,
    pub frames: Vec<Vec<Mat4>>, // A vector of frames, where each frame contains a vector of bone transforms
}

#[derive(Debug, Clone, PartialEq)]
pub struct TrackSample {
    pub time: f32,
    pub bone_transforms: Vec<Mat4>,
}

impl AnimationTrack {
    pub fn new(duration_seconds: f32, frames_per_second: f32, is_looping: bool, frames: Vec<Vec<Mat4>>) -> Self {
        Self {
            duration_seconds,
            frames_per_second,
            is_looping,
            frames,
        }
    }
}

pub fn calculate_loop_time(current_time: f32, duration: f32, loop_flag: bool) -> f32 {
    if duration <= 0.0 {
        return 0.0;
    }

    if loop_flag {
        let mut time = current_time % duration;
        if time < 0.0 {
            time += duration;
        }
        time
    } else {
        if current_time < 0.0 {
            0.0
        } else if current_time > duration {
            duration
        } else {
            current_time
        }
    }
}

pub fn evaluate_animation_at_time(track: &AnimationTrack, current_time: f32) -> TrackSample {
    let time = calculate_loop_time(current_time, track.duration_seconds, track.is_looping);
    
    if track.frames.is_empty() {
        return TrackSample {
            time,
            bone_transforms: Vec::new(),
        };
    }
    
    if track.frames.len() == 1 {
        return TrackSample {
            time,
            bone_transforms: track.frames[0].clone(),
        };
    }

    let frame_time = 1.0 / track.frames_per_second;
    let exact_frame = time / frame_time;
    let frame0_index = exact_frame.floor() as usize;
    let t = exact_frame - frame0_index as f32;
    
    let total_frames = track.frames.len();
    
    let f0 = if frame0_index >= total_frames { total_frames - 1 } else { frame0_index };
    let f1 = if f0 + 1 >= total_frames {
        if track.is_looping {
            0
        } else {
            f0
        }
    } else {
        f0 + 1
    };

    if f0 == f1 || t == 0.0 {
        return TrackSample {
            time,
            bone_transforms: track.frames[f0].clone(),
        };
    }

    let frame0_bones = &track.frames[f0];
    let frame1_bones = &track.frames[f1];
    
    let mut interpolated_bones = Vec::with_capacity(frame0_bones.len());
    
    for (m0, m1) in frame0_bones.iter().zip(frame1_bones.iter()) {
        let (s0, q0, t0) = m0.to_scale_rotation_translation();
        let (s1, q1, t1) = m1.to_scale_rotation_translation();
        
        let s_interp = s0.lerp(s1, t);
        let q_interp = q0.slerp(q1, t).normalize();
        let t_interp = t0.lerp(t1, t);
        
        let m_interp = Mat4::from_scale_rotation_translation(s_interp, q_interp, t_interp);
        interpolated_bones.push(m_interp);
    }

    TrackSample {
        time,
        bone_transforms: interpolated_bones,
    }
}
