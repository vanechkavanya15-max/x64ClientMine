use std::collections::HashMap;
use std::sync::{OnceLock, RwLock};
use std::panic::{catch_unwind, AssertUnwindSafe};
use crate::character_manager_core::get_character_manager;

#[derive(Debug, Clone, Copy)]
pub struct Actor {
    pub vid: u32,
    pub race: u32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub rot: f32,
    pub time_alive_ms: u32,
}

#[no_mangle]
pub extern "C" fn m2_chr_create(vid: u32, race: u32, x: f32, y: f32, z: f32, rot: f32) -> bool {
    catch_unwind(AssertUnwindSafe(|| {
        let mut mgr = get_character_manager().write().unwrap();
        if mgr.create_instance(vid, race as u16, x, y, z) {
            if let Some(inst) = mgr.get_instance_mut(vid) {
                inst.rot_deg = rot;
            }
            true
        } else {
            false
        }
    })).unwrap_or(false)
}

#[no_mangle]
pub extern "C" fn m2_chr_delete(vid: u32) -> bool {
    catch_unwind(AssertUnwindSafe(|| {
        let mut mgr = get_character_manager().write().unwrap();
        mgr.delete_instance(vid)
    })).unwrap_or(false)
}

#[no_mangle]
pub extern "C" fn m2_chr_get_count() -> u32 {
    catch_unwind(AssertUnwindSafe(|| {
        let mgr = get_character_manager().read().unwrap();
        mgr.count() as u32
    })).unwrap_or(0)
}

#[no_mangle]
pub extern "C" fn m2_chr_update(delta_time_ms: u32) {
    let _ = catch_unwind(AssertUnwindSafe(|| {
        let mut mgr = get_character_manager().write().unwrap();
        for inst in mgr.alive_instances.values_mut() {
            inst.time_alive_ms = inst.time_alive_ms.saturating_add(delta_time_ms);
        }
    }));
}

pub fn test_get_actor(vid: u32) -> Option<Actor> {
    let mgr = get_character_manager().read().unwrap();
    mgr.get_instance(vid).map(|inst| Actor {
        vid: inst.vid,
        race: inst.race as u32,
        x: inst.x,
        y: inst.y,
        z: inst.z,
        rot: inst.rot_deg,
        time_alive_ms: inst.time_alive_ms,
    })
}

pub fn test_clear_actors() {
    let mut mgr = get_character_manager().write().unwrap();
    mgr.clear();
}

pub fn get_actor_pos(vid: u32) -> Option<(f32, f32, f32)> {
    let mgr = get_character_manager().read().unwrap();
    mgr.get_instance(vid).map(|inst| (inst.x, inst.y, inst.z))
}
