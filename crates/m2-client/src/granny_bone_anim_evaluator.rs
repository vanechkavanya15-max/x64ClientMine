use glam::{Mat4, Quat, Vec3};
use std::collections::HashMap;

/// A position keyframe for a bone animation track.
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct PositionKey {
    pub time: f32,
    pub position: Vec3,
}

/// A rotation keyframe for a bone animation track.
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct RotationKey {
    pub time: f32,
    pub rotation: Quat,
}

/// An animation track for a specific bone.
#[derive(Debug, Clone, PartialEq)]
pub struct BoneTrack {
    pub bone_index: usize,
    pub position_keys: Vec<PositionKey>,
    pub rotation_keys: Vec<RotationKey>,
}

/// A full animation for a model.
#[derive(Debug, Clone, PartialEq)]
pub struct GrannyAnimation {
    pub name: String,
    pub duration: f32,
    pub tracks: Vec<BoneTrack>,
}

/// Evaluator state for computing bone transforms and skinning matrices.
#[derive(Debug, Clone)]
pub struct GrannyBoneAnimEvaluator {
    // Current local transforms (evaluated from animation)
    pub local_transforms: HashMap<usize, Mat4>,
    // Current global transforms (world transforms)
    pub global_transforms: HashMap<usize, Mat4>,
    // Final skin matrices
    pub skin_matrices: HashMap<usize, Mat4>,
    
    // Parent hierarchy: bone_index -> parent_bone_index
    pub hierarchy: HashMap<usize, usize>,
    
    // Inverse bind poses: bone_index -> inv_bind_matrix
    pub inv_bind_poses: HashMap<usize, Mat4>,
    
    // Rest poses (base local transforms): bone_index -> local_rest_matrix
    pub rest_poses: HashMap<usize, Mat4>,
}

impl GrannyBoneAnimEvaluator {
    pub fn new() -> Self {
        Self {
            local_transforms: HashMap::new(),
            global_transforms: HashMap::new(),
            skin_matrices: HashMap::new(),
            hierarchy: HashMap::new(),
            inv_bind_poses: HashMap::new(),
            rest_poses: HashMap::new(),
        }
    }

    /// Evaluates a specific track at a given time `t` to produce a local transform matrix.
    fn evaluate_track(&self, track: &BoneTrack, t: f32) -> Mat4 {
        let position = Self::evaluate_position(&track.position_keys, t);
        let rotation = Self::evaluate_rotation(&track.rotation_keys, t);
        
        Mat4::from_scale_rotation_translation(Vec3::ONE, rotation, position)
    }

    /// Helper to LERP position.
    fn evaluate_position(keys: &[PositionKey], t: f32) -> Vec3 {
        if keys.is_empty() {
            return Vec3::ZERO;
        }
        if keys.len() == 1 || t <= keys[0].time {
            return keys[0].position;
        }
        let last_idx = keys.len() - 1;
        if t >= keys[last_idx].time {
            return keys[last_idx].position;
        }

        for i in 0..keys.len() - 1 {
            let k0 = &keys[i];
            let k1 = &keys[i + 1];
            if t >= k0.time && t <= k1.time {
                let factor = (t - k0.time) / (k1.time - k0.time);
                return k0.position.lerp(k1.position, factor);
            }
        }
        
        keys[last_idx].position
    }

    /// Helper to SLERP rotation.
    fn evaluate_rotation(keys: &[RotationKey], t: f32) -> Quat {
        if keys.is_empty() {
            return Quat::IDENTITY;
        }
        if keys.len() == 1 || t <= keys[0].time {
            return keys[0].rotation;
        }
        let last_idx = keys.len() - 1;
        if t >= keys[last_idx].time {
            return keys[last_idx].rotation;
        }

        for i in 0..keys.len() - 1 {
            let k0 = &keys[i];
            let k1 = &keys[i + 1];
            if t >= k0.time && t <= k1.time {
                let factor = (t - k0.time) / (k1.time - k0.time);
                // SLERP natively interpolates along the shortest path
                return k0.rotation.slerp(k1.rotation, factor).normalize();
            }
        }
        
        keys[last_idx].rotation
    }

    /// Update local transforms for a single animation.
    pub fn evaluate_animation(&mut self, anim: &GrannyAnimation, t: f32) {
        for track in &anim.tracks {
            self.local_transforms.insert(track.bone_index, self.evaluate_track(track, t));
        }
    }

    /// Smoothly blend between two animations and update local transforms.
    /// `blend_factor` is 0.0 for `anim_a` fully, and 1.0 for `anim_b` fully.
    pub fn blend_animations(&mut self, anim_a: &GrannyAnimation, t_a: f32, anim_b: &GrannyAnimation, t_b: f32, blend_factor: f32) {
        let blend_factor = blend_factor.clamp(0.0, 1.0);
        
        // Collect all bone indices involved
        let mut bone_indices = Vec::new();
        for track in &anim_a.tracks {
            if !bone_indices.contains(&track.bone_index) {
                bone_indices.push(track.bone_index);
            }
        }
        for track in &anim_b.tracks {
            if !bone_indices.contains(&track.bone_index) {
                bone_indices.push(track.bone_index);
            }
        }

        for bone_index in bone_indices {
            let track_a = anim_a.tracks.iter().find(|t| t.bone_index == bone_index);
            let track_b = anim_b.tracks.iter().find(|t| t.bone_index == bone_index);
            
            let fallback_tf = self.rest_poses.get(&bone_index).copied().unwrap_or(Mat4::IDENTITY);
            let (_, fallback_rot, fallback_pos) = fallback_tf.to_scale_rotation_translation();
            
            let pos_a = if let Some(track) = track_a {
                Self::evaluate_position(&track.position_keys, t_a)
            } else {
                fallback_pos
            };
            
            let pos_b = if let Some(track) = track_b {
                Self::evaluate_position(&track.position_keys, t_b)
            } else {
                fallback_pos
            };
            
            let rot_a = if let Some(track) = track_a {
                Self::evaluate_rotation(&track.rotation_keys, t_a)
            } else {
                fallback_rot
            };
            
            let rot_b = if let Some(track) = track_b {
                Self::evaluate_rotation(&track.rotation_keys, t_b)
            } else {
                fallback_rot
            };
            
            let blended_pos = pos_a.lerp(pos_b, blend_factor);
            let blended_rot = rot_a.slerp(rot_b, blend_factor).normalize();
            
            let local_transform = Mat4::from_scale_rotation_translation(Vec3::ONE, blended_rot, blended_pos);
            self.local_transforms.insert(bone_index, local_transform);
        }
    }

    /// Compute global transforms based on parent hierarchy.
    /// Hierarchy should be populated before calling this.
    /// WorldTransform = ParentTransform * LocalTransform
    pub fn calculate_global_transforms(&mut self) {
        // Collect all bone indices, from local_transforms and rest_poses, and hierarchy values (parents might only exist in rest_poses)
        let mut bones_set = std::collections::HashSet::new();
        for &k in self.local_transforms.keys() { bones_set.insert(k); }
        for &k in self.rest_poses.keys() { bones_set.insert(k); }
        for &k in self.hierarchy.keys() { bones_set.insert(k); }
        for &v in self.hierarchy.values() { bones_set.insert(v); }
        
        let mut bones: Vec<usize> = bones_set.into_iter().collect();
        // Simple sort to try to process parents before children if IDs are sequentially generated.
        bones.sort();
        
        let mut updated_globals = HashMap::new();
        
        let mut pending = bones.clone();
        let mut progress = true;
        
        while !pending.is_empty() && progress {
            progress = false;
            let mut next_pending = Vec::new();
            
            for &bone_index in &pending {
                let local_tf = self.local_transforms.get(&bone_index)
                    .copied()
                    .or_else(|| self.rest_poses.get(&bone_index).copied())
                    .unwrap_or(Mat4::IDENTITY);
                
                if let Some(&parent_index) = self.hierarchy.get(&bone_index) {
                    if let Some(&parent_global) = updated_globals.get(&parent_index) {
                        let global_tf = parent_global * local_tf;
                        updated_globals.insert(bone_index, global_tf);
                        progress = true;
                    } else {
                        next_pending.push(bone_index);
                    }
                } else {
                    // Root bone
                    updated_globals.insert(bone_index, local_tf);
                    progress = true;
                }
            }
            pending = next_pending;
        }
        
        self.global_transforms = updated_globals;
    }

    /// Compute final skinning matrices for rendering.
    /// SkinMatrix[i] = CurrentBoneTransform[i] * InverseBindPose[i]
    pub fn compute_skin_matrices(&mut self) {
        for (bone_index, global_tf) in &self.global_transforms {
            let inv_bind = self.inv_bind_poses.get(bone_index).copied().unwrap_or(Mat4::IDENTITY);
            let skin_tf = *global_tf * inv_bind;
            self.skin_matrices.insert(*bone_index, skin_tf);
        }
    }
}
