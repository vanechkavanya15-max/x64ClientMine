use glam::{Quat, Vec3};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[repr(C)]
#[derive(Clone, Debug)]
pub struct TranslationKey {
    pub time: f32,
    pub val: Vec3,
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct RotationKey {
    pub time: f32,
    pub val: Quat,
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct BoneAnimation {
    pub translation_keys: Vec<TranslationKey>,
    pub rotation_keys: Vec<RotationKey>,
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct AnimationCurve {
    pub anim_id: u32,
    pub bones: HashMap<u16, BoneAnimation>,
}

lazy_static::lazy_static! {
    static ref ANIMATION_REGISTRY: Arc<RwLock<HashMap<u32, AnimationCurve>>> = Arc::new(RwLock::new(HashMap::new()));
}

pub fn register_animation(anim: AnimationCurve) {
    if let Ok(mut registry) = ANIMATION_REGISTRY.write() {
        registry.insert(anim.anim_id, anim);
    }
}

pub fn clear_animations() {
    if let Ok(mut registry) = ANIMATION_REGISTRY.write() {
        registry.clear();
    }
}

fn sample_translation(keys: &[TranslationKey], time: f32) -> Vec3 {
    if keys.is_empty() {
        return Vec3::ZERO;
    }
    if keys.len() == 1 || time <= keys[0].time {
        return keys[0].val;
    }
    let last_idx = keys.len() - 1;
    if time >= keys[last_idx].time {
        return keys[last_idx].val;
    }

    // Find the segment
    for i in 0..keys.len() - 1 {
        if time >= keys[i].time && time <= keys[i + 1].time {
            let t0 = keys[i].time;
            let t1 = keys[i + 1].time;
            let v0 = keys[i].val;
            let v1 = keys[i + 1].val;

            let t = (time - t0) / (t1 - t0);
            return v0.lerp(v1, t);
        }
    }
    
    keys[last_idx].val
}

fn sample_rotation(keys: &[RotationKey], time: f32) -> Quat {
    if keys.is_empty() {
        return Quat::IDENTITY;
    }
    if keys.len() == 1 || time <= keys[0].time {
        return keys[0].val;
    }
    let last_idx = keys.len() - 1;
    if time >= keys[last_idx].time {
        return keys[last_idx].val;
    }

    // Find the segment
    for i in 0..keys.len() - 1 {
        if time >= keys[i].time && time <= keys[i + 1].time {
            let t0 = keys[i].time;
            let t1 = keys[i + 1].time;
            let q0 = keys[i].val;
            let q1 = keys[i + 1].val;

            let t = (time - t0) / (t1 - t0);
            return q0.slerp(q1, t).normalize();
        }
    }
    keys[last_idx].val
}

#[allow(improper_ctypes_definitions)]
#[no_mangle]
pub extern "C" fn m2_motion_eval_bone(anim_id: u32, bone_id: u16, time: f32) -> (f32, f32, f32, f32, f32, f32, f32) {
    if let Ok(registry) = ANIMATION_REGISTRY.read() {
        if let Some(anim) = registry.get(&anim_id) {
            if let Some(bone) = anim.bones.get(&bone_id) {
                let trans = sample_translation(&bone.translation_keys, time);
                let rot = sample_rotation(&bone.rotation_keys, time);
                return (trans.x, trans.y, trans.z, rot.x, rot.y, rot.z, rot.w);
            }
        }
    }
    // Default fallback if not found
    (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0)
}
