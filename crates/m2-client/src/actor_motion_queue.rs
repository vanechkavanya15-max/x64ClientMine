use std::collections::HashMap;
use std::sync::{OnceLock, RwLock};
use std::panic::{catch_unwind, AssertUnwindSafe};

pub const MOTION_WAIT: u8 = 1;
pub const MOTION_WALK: u8 = 2;
pub const MOTION_RUN: u8 = 3;
pub const MOTION_NORMAL_ATTACK: u8 = 4;
pub const MOTION_DAMAGE: u8 = 5;
pub const MOTION_DEAD: u8 = 11;
pub const MOTION_COMBO_ATTACK: u8 = 14;
pub const MOTION_RESPAWN: u8 = 15;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActorMotionQueue {
    pub cur_motion_id: u8,
    pub motion_duration_ms: u32,
    pub motion_elapsed_ms: u32,
    pub is_looping: bool,
}

impl ActorMotionQueue {
    pub fn new() -> Self {
        Self {
            cur_motion_id: MOTION_WAIT,
            motion_duration_ms: 0,
            motion_elapsed_ms: 0,
            is_looping: true,
        }
    }

    pub fn set_motion(&mut self, motion_id: u8, duration_ms: u32, is_looping: bool) -> bool {
        if self.cur_motion_id == MOTION_DEAD && motion_id != MOTION_DEAD && motion_id != MOTION_RESPAWN {
            return false;
        }

        if self.cur_motion_id == MOTION_DAMAGE && self.motion_elapsed_ms < self.motion_duration_ms {
            if motion_id == MOTION_WALK || motion_id == MOTION_RUN || motion_id == MOTION_WAIT {
                return false;
            }
        }
        
        if self.cur_motion_id == MOTION_COMBO_ATTACK && self.motion_elapsed_ms < self.motion_duration_ms {
             if motion_id == MOTION_WALK || motion_id == MOTION_RUN || motion_id == MOTION_WAIT {
                 return false;
             }
        }

        if self.cur_motion_id == MOTION_NORMAL_ATTACK && self.motion_elapsed_ms < self.motion_duration_ms {
            if motion_id != MOTION_DAMAGE && motion_id != MOTION_DEAD && motion_id != MOTION_RESPAWN {
                 return false;
            }
        }

        self.cur_motion_id = motion_id;
        self.motion_duration_ms = duration_ms;
        self.motion_elapsed_ms = 0;
        self.is_looping = is_looping;
        true
    }

    pub fn update_motion(&mut self, delta_time_ms: u32) -> bool {
        if self.is_looping {
            self.motion_elapsed_ms = self.motion_elapsed_ms.saturating_add(delta_time_ms);
            if self.motion_duration_ms > 0 {
                self.motion_elapsed_ms %= self.motion_duration_ms;
            }
            return false;
        }

        let old_elapsed = self.motion_elapsed_ms;
        self.motion_elapsed_ms = self.motion_elapsed_ms.saturating_add(delta_time_ms);

        if self.motion_duration_ms == 0 && old_elapsed == 0 && self.motion_elapsed_ms > 0 {
            return true;
        }
        
        if old_elapsed < self.motion_duration_ms && self.motion_elapsed_ms >= self.motion_duration_ms {
            return true;
        }

        false
    }
}

use crate::character_manager_core::get_character_manager;

#[no_mangle]
pub extern "C" fn m2_motion_set_state(vid: u32, motion_type: u8) -> bool {
    catch_unwind(AssertUnwindSafe(|| {
        let mut mgr = get_character_manager().write().unwrap();
        if let Some(inst) = mgr.get_instance_mut(vid) {
            let duration = if motion_type == MOTION_DEAD { 2000 } else { 1000 };
            inst.motion_queue.set_motion(motion_type, duration, false)
        } else {
            false
        }
    })).unwrap_or(false)
}

#[no_mangle]
pub extern "C" fn m2_motion_get_state(vid: u32) -> u8 {
    catch_unwind(AssertUnwindSafe(|| {
        let mgr = get_character_manager().read().unwrap();
        if let Some(inst) = mgr.get_instance(vid) {
            inst.motion_queue.cur_motion_id
        } else {
            MOTION_WAIT
        }
    })).unwrap_or(MOTION_WAIT)
}

#[no_mangle]
pub extern "C" fn m2_motion_is_dead(vid: u32) -> bool {
    catch_unwind(AssertUnwindSafe(|| {
        let mgr = get_character_manager().read().unwrap();
        if let Some(inst) = mgr.get_instance(vid) {
            inst.motion_queue.cur_motion_id == MOTION_DEAD
        } else {
            false
        }
    })).unwrap_or(false)
}
