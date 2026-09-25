use std::collections::HashMap;
use crate::ffi_guard::safe_ffi_call;

pub const SKILLS_CONTAINER: u32 = 0;

#[derive(Debug, Clone)]
pub struct SkillEntry {
    pub vnum: u32,
    pub level: u8,
    pub next_use_time: u32,
}

#[derive(Debug, Clone, Default)]
pub struct SkillsContainer {
    skills: HashMap<u32, SkillEntry>,
}

impl SkillsContainer {
    pub fn new() -> Self {
        Self {
            skills: HashMap::new(),
        }
    }

    pub fn set_skill(&mut self, vnum: u32, level: u8) {
        if let Some(skill) = self.skills.get_mut(&vnum) {
            skill.level = level;
        } else {
            self.skills.insert(vnum, SkillEntry {
                vnum,
                level,
                next_use_time: 0,
            });
        }
    }

    pub fn can_use(&self, vnum: u32, current_time: u32) -> bool {
        if let Some(skill) = self.skills.get(&vnum) {
            if skill.level > 0 {
                return current_time >= skill.next_use_time;
            }
        }
        false
    }

    pub fn trigger_cooldown(&mut self, vnum: u32, cd_ms: u32, current_time: u32) {
        if let Some(skill) = self.skills.get_mut(&vnum) {
            skill.next_use_time = current_time.wrapping_add(cd_ms);
        }
    }
}

// Globalny wskaznik na kontener umiejetnosci gracza do obslugi m2_skill_can_use
static mut GLOBAL_PLAYER_SKILLS: *mut SkillsContainer = std::ptr::null_mut();

#[no_mangle]
pub unsafe extern "C" fn m2_skills_container_new() -> *mut SkillsContainer {
    safe_ffi_call(std::ptr::null_mut(), || {
        let container = Box::new(SkillsContainer::new());
        Box::into_raw(container)
    })
}

#[no_mangle]
pub unsafe extern "C" fn m2_skills_container_free(ptr: *mut SkillsContainer) {
    safe_ffi_call((), || {
        if !ptr.is_null() {
            let _ = Box::from_raw(ptr);
        }
    })
}

#[no_mangle]
pub unsafe extern "C" fn m2_skills_container_set_skill(ptr: *mut SkillsContainer, vnum: u32, level: u8) {
    safe_ffi_call((), || {
        if !ptr.is_null() {
            (*ptr).set_skill(vnum, level);
        }
    })
}

#[no_mangle]
pub unsafe extern "C" fn m2_skills_container_can_use(ptr: *const SkillsContainer, vnum: u32, current_time: u32) -> bool {
    safe_ffi_call(false, || {
        if !ptr.is_null() {
            (*ptr).can_use(vnum, current_time)
        } else {
            false
        }
    })
}

#[no_mangle]
pub unsafe extern "C" fn m2_skills_container_trigger_cooldown(ptr: *mut SkillsContainer, vnum: u32, cd_ms: u32, current_time: u32) {
    safe_ffi_call((), || {
        if !ptr.is_null() {
            (*ptr).trigger_cooldown(vnum, cd_ms, current_time);
        }
    })
}

// Wymagany przez zadanie eksport C-FFI dla m2_skill_can_use
#[no_mangle]
pub unsafe extern "C" fn m2_skill_can_use(vnum: u32) -> bool {
    safe_ffi_call(false, || {
        if !GLOBAL_PLAYER_SKILLS.is_null() {
            (*GLOBAL_PLAYER_SKILLS).can_use(vnum, crate::timer::get_msec())
        } else {
            false
        }
    })
}

// Funkcja pomocnicza do ustawienia globalnego kontenera, aby m2_skill_can_use mogl dzialac
#[no_mangle]
pub unsafe extern "C" fn m2_skills_set_global_container(ptr: *mut SkillsContainer) {
    safe_ffi_call((), || {
        GLOBAL_PLAYER_SKILLS = ptr;
    })
}
