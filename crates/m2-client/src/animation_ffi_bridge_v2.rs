use std::collections::HashMap;
use std::sync::{OnceLock, RwLock};

#[derive(Debug, Clone)]
pub struct AnimationState {
    pub motion_id: u8,
    pub is_loop: bool,
    pub current_time: f32,
    pub duration: f32,
    pub attached_weapon: u32,
    pub last_event_type: u8,
}

impl Default for AnimationState {
    fn default() -> Self {
        Self {
            motion_id: 0,
            is_loop: false,
            current_time: 0.0,
            duration: 1.0, // Default duration
            attached_weapon: 0,
            last_event_type: 0,
        }
    }
}

pub struct AnimationManager {
    states: HashMap<u32, AnimationState>,
}

impl AnimationManager {
    pub fn new() -> Self {
        Self {
            states: HashMap::new(),
        }
    }

    pub fn play(&mut self, vid: u32, motion_id: u8, is_loop: bool) -> bool {
        let state = self.states.entry(vid).or_default();
        state.motion_id = motion_id;
        state.is_loop = is_loop;
        state.current_time = 0.0;
        state.last_event_type = 0; // reset event
        true
    }

    pub fn update(&mut self, vid: u32, dt: f32) -> bool {
        if let Some(state) = self.states.get_mut(&vid) {
            state.current_time += dt;
            if state.current_time >= state.duration {
                if state.is_loop {
                    state.current_time %= state.duration;
                } else {
                    state.current_time = state.duration; // Clamp
                    state.last_event_type = 1; // End event
                }
            }
            return true;
        }
        false
    }

    pub fn get_event(&self, vid: u32, out_event_type: &mut u8) -> bool {
        if let Some(state) = self.states.get(&vid) {
            *out_event_type = state.last_event_type;
            return true;
        }
        false
    }

    pub fn attach_weapon(&mut self, vid: u32, weapon_vnum: u32) -> bool {
        let state = self.states.entry(vid).or_default();
        state.attached_weapon = weapon_vnum;
        true
    }
}

static ANIM_MGR: OnceLock<RwLock<AnimationManager>> = OnceLock::new();

fn get_anim_mgr() -> &'static RwLock<AnimationManager> {
    ANIM_MGR.get_or_init(|| RwLock::new(AnimationManager::new()))
}

#[no_mangle]
pub extern "C" fn m2_anim_play(vid: u32, motion_id: u8, is_loop: bool) -> bool {
    std::panic::catch_unwind(|| {
        if let Ok(mut mgr) = get_anim_mgr().write() {
            mgr.play(vid, motion_id, is_loop)
        } else {
            false
        }
    }).unwrap_or(false)
}

#[no_mangle]
pub extern "C" fn m2_anim_update(vid: u32, dt: f32) -> bool {
    std::panic::catch_unwind(|| {
        if let Ok(mut mgr) = get_anim_mgr().write() {
            mgr.update(vid, dt)
        } else {
            false
        }
    }).unwrap_or(false)
}

#[no_mangle]
pub extern "C" fn m2_anim_get_event(vid: u32, out_event_type: *mut u8) -> bool {
    if out_event_type.is_null() {
        return false;
    }
    std::panic::catch_unwind(|| {
        if let Ok(mgr) = get_anim_mgr().read() {
            // SAFETY: out_event_type is checked for null, and we are within catch_unwind
            let event_type_ref = unsafe { &mut *out_event_type };
            mgr.get_event(vid, event_type_ref)
        } else {
            false
        }
    }).unwrap_or(false)
}

#[no_mangle]
pub extern "C" fn m2_anim_attach_weapon(vid: u32, weapon_vnum: u32) -> bool {
    std::panic::catch_unwind(|| {
        if let Ok(mut mgr) = get_anim_mgr().write() {
            mgr.attach_weapon(vid, weapon_vnum)
        } else {
            false
        }
    }).unwrap_or(false)
}
