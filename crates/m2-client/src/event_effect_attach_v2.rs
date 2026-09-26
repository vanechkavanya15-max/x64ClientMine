use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

use m2_client_bridge::ffi_guard::safe_ffi_call;

#[derive(Debug, Clone)]
pub struct MotionEffectEventData {
    pub is_independent: bool,
    pub is_attaching: bool,
    pub is_following: bool,
    pub effect_index: u32,
    pub attaching_bone_name: String,
    pub effect_position: (f32, f32, f32),
}

#[derive(Debug, Clone)]
pub struct AttachingEffect {
    pub effect_index: u32,
    pub bone_index: i32,
    pub is_attaching: bool,
    pub is_independent: bool,
    pub is_following: bool,
    pub effect_position: (f32, f32, f32),
    pub bone_name: String,
}

pub struct ActorEffectState {
    pub attached_effects: Vec<AttachingEffect>,
}

pub struct EffectSystemManager {
    actors: HashMap<u32, ActorEffectState>,
    bone_map: HashMap<String, i32>,
}

impl EffectSystemManager {
    pub fn new() -> Self {
        Self {
            actors: HashMap::new(),
            bone_map: HashMap::new(),
        }
    }

    pub fn register_bone(&mut self, bone_name: String, bone_index: i32) {
        self.bone_map.insert(bone_name, bone_index);
    }

    pub fn get_bone_index(&self, bone_name: &str) -> Option<i32> {
        self.bone_map.get(bone_name).copied()
    }

    pub fn process_event(&mut self, actor_vid: u32, data: MotionEffectEventData) {
        let actor = self.actors.entry(actor_vid).or_insert(ActorEffectState {
            attached_effects: Vec::new(),
        });

        if data.is_independent {
            actor.attached_effects.push(AttachingEffect {
                effect_index: data.effect_index,
                bone_index: -1,
                is_attaching: false,
                is_independent: true,
                is_following: false,
                effect_position: data.effect_position,
                bone_name: String::new(),
            });
            return;
        }

        if data.is_attaching {
            if data.is_following {
                let bone_index = self.bone_map.get(&data.attaching_bone_name).copied().unwrap_or(-1);
                actor.attached_effects.push(AttachingEffect {
                    effect_index: data.effect_index,
                    bone_index,
                    is_attaching: true,
                    is_independent: false,
                    is_following: true,
                    effect_position: data.effect_position,
                    bone_name: data.attaching_bone_name,
                });
            } else {
                if let Some(bone_index) = self.bone_map.get(&data.attaching_bone_name).copied() {
                    actor.attached_effects.push(AttachingEffect {
                        effect_index: data.effect_index,
                        bone_index,
                        is_attaching: true,
                        is_independent: false,
                        is_following: false,
                        effect_position: data.effect_position,
                        bone_name: data.attaching_bone_name,
                    });
                }
            }
        } else {
            actor.attached_effects.push(AttachingEffect {
                effect_index: data.effect_index,
                bone_index: -1,
                is_attaching: false,
                is_independent: false,
                is_following: false,
                effect_position: data.effect_position,
                bone_name: String::new(),
            });
        }
    }

    pub fn clear_events(&mut self, actor_vid: u32) {
        if let Some(actor) = self.actors.get_mut(&actor_vid) {
            actor.attached_effects.clear();
        }
    }
}

static EFFECT_MANAGER: OnceLock<Mutex<EffectSystemManager>> = OnceLock::new();

pub fn get_effect_manager() -> std::sync::MutexGuard<'static, EffectSystemManager> {
    EFFECT_MANAGER
        .get_or_init(|| Mutex::new(EffectSystemManager::new()))
        .lock()
        .unwrap()
}

#[repr(C, packed)]
pub struct FfiMotionEffectEventData {
    pub type_id: i32,
    pub frame_time: u32,
    pub is_independent: i32, // BOOL
    pub is_attaching: i32, // BOOL
    pub is_following: i32, // BOOL
    pub effect_index: u32,
    pub attaching_bone_name: [u8; 32],
    pub effect_position_x: f32,
    pub effect_position_y: f32,
    pub effect_position_z: f32,
}

#[no_mangle]
pub extern "C" fn m2_motion_process_effect_event(
    actor_vid: u32,
    event_data: *const FfiMotionEffectEventData,
) -> bool {
    safe_ffi_call(false, || {
        if event_data.is_null() {
            return false;
        }

        let ffi_data = unsafe { std::ptr::addr_of!(*event_data).read_unaligned() };

        let bone_name_len = ffi_data
            .attaching_bone_name
            .iter()
            .position(|&c| c == 0)
            .unwrap_or(32);
        let attaching_bone_name = std::str::from_utf8(&ffi_data.attaching_bone_name[..bone_name_len])
            .unwrap_or("")
            .to_string();

        let data = MotionEffectEventData {
            is_independent: ffi_data.is_independent != 0,
            is_attaching: ffi_data.is_attaching != 0,
            is_following: ffi_data.is_following != 0,
            effect_index: ffi_data.effect_index,
            attaching_bone_name,
            effect_position: (
                ffi_data.effect_position_x,
                ffi_data.effect_position_y,
                ffi_data.effect_position_z,
            ),
        };

        let mut manager = get_effect_manager();
        manager.process_event(actor_vid, data);
        true
    })
}

#[no_mangle]
pub extern "C" fn m2_motion_register_bone(
    bone_name: *const std::ffi::c_char,
    bone_index: i32,
) -> bool {
    safe_ffi_call(false, || {
        if bone_name.is_null() {
            return false;
        }

        let c_str = unsafe { std::ffi::CStr::from_ptr(bone_name) };
        let bone_name_str = c_str.to_string_lossy().into_owned();

        let mut manager = get_effect_manager();
        manager.register_bone(bone_name_str, bone_index);
        true
    })
}

#[repr(C)]
pub struct FfiAttachingEffect {
    pub effect_index: u32,
    pub bone_index: i32,
    pub is_attaching: i32, // BOOL
    pub is_independent: i32, // BOOL
    pub is_following: i32, // BOOL
    pub effect_position_x: f32,
    pub effect_position_y: f32,
    pub effect_position_z: f32,
}

#[no_mangle]
pub extern "C" fn m2_motion_get_attached_effects(
    actor_vid: u32,
    out_effects: *mut FfiAttachingEffect,
    max_count: usize,
) -> usize {
    safe_ffi_call(0, || {
        if out_effects.is_null() || max_count == 0 {
            return 0;
        }

        let mut manager = get_effect_manager();
        
        let count = if let Some(actor) = manager.actors.get_mut(&actor_vid) {
            let n = std::cmp::min(actor.attached_effects.len(), max_count);
            let out_slice = unsafe { std::slice::from_raw_parts_mut(out_effects, n) };

            for (i, effect) in actor.attached_effects.iter().take(n).enumerate() {
                out_slice[i] = FfiAttachingEffect {
                    effect_index: effect.effect_index,
                    bone_index: effect.bone_index,
                    is_attaching: if effect.is_attaching { 1 } else { 0 },
                    is_independent: if effect.is_independent { 1 } else { 0 },
                    is_following: if effect.is_following { 1 } else { 0 },
                    effect_position_x: effect.effect_position.0,
                    effect_position_y: effect.effect_position.1,
                    effect_position_z: effect.effect_position.2,
                };
            }
            // Clear effects immediately after providing them, preventing memory leaks
            actor.attached_effects.clear();
            n
        } else {
            0
        };

        count
    })
}
