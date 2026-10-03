use std::collections::HashMap;
use std::ffi::CString;
use std::os::raw::c_char;
use std::sync::{Mutex, OnceLock};

use crate::skill_calculator::{PlayerClass, PlayerStats, SkillCalculator};
use crate::skill_table_core::{SkillData, SkillTableCore, SKILL_TYPE_ATTACK, SKILL_TYPE_BUFF, SKILL_TYPE_TOGGLE};

#[derive(Debug, Clone)]
pub struct PySkillDesc {
    pub name: CString,
    pub description: CString,
}

pub struct PySkillBridge {
    table: SkillTableCore,
    descs: HashMap<u32, PySkillDesc>,
    grade_names: HashMap<u8, CString>,
}

impl PySkillBridge {
    pub fn new() -> Self {
        let mut bridge = Self {
            table: SkillTableCore::new(),
            descs: HashMap::new(),
            grade_names: HashMap::new(),
        };

        bridge.grade_names.insert(0, CString::new("Normal").unwrap());
        bridge.grade_names.insert(1, CString::new("Master").unwrap());
        bridge.grade_names.insert(2, CString::new("Grand Master").unwrap());
        bridge.grade_names.insert(3, CString::new("Perfect Master").unwrap());

        bridge
    }

    pub fn register_skill(&mut self, data: SkillData, name: &str, desc: &str) {
        self.table.register_skill(data.clone());
        self.descs.insert(data.skill_vnum, PySkillDesc {
            name: CString::new(name).unwrap_or_default(),
            description: CString::new(desc).unwrap_or_default(),
        });
    }

    pub fn get_skill_name(&self, skill_id: u32) -> *const c_char {
        if let Some(desc) = self.descs.get(&skill_id) {
            desc.name.as_ptr()
        } else {
            std::ptr::null()
        }
    }

    pub fn get_skill_description(&self, skill_id: u32) -> *const c_char {
        if let Some(desc) = self.descs.get(&skill_id) {
            desc.description.as_ptr()
        } else {
            std::ptr::null()
        }
    }

    pub fn get_skill_type(&self, skill_id: u32) -> u8 {
        if let Some(skill) = self.table.get_skill(skill_id) {
            skill.skill_type
        } else {
            0
        }
    }

    pub fn get_skill_max_level(&self, skill_id: u32) -> u8 {
        if let Some(skill) = self.table.get_skill(skill_id) {
            skill.max_level
        } else {
            0
        }
    }

    pub fn get_skill_grade_name(&self, grade_id: u8) -> *const c_char {
        if let Some(name) = self.grade_names.get(&grade_id) {
            name.as_ptr()
        } else {
            std::ptr::null()
        }
    }
}

impl Default for PySkillBridge {
    fn default() -> Self {
        Self::new()
    }
}

static PY_SKILL_BRIDGE: OnceLock<Mutex<PySkillBridge>> = OnceLock::new();

pub fn get_bridge() -> std::sync::MutexGuard<'static, PySkillBridge> {
    PY_SKILL_BRIDGE
        .get_or_init(|| Mutex::new(PySkillBridge::new()))
        .lock()
        .unwrap()
}

// FFI Exports

#[no_mangle]
pub extern "C" fn m2_py_skill_get_name(skill_id: u32) -> *const c_char {
    get_bridge().get_skill_name(skill_id)
}

#[no_mangle]
pub extern "C" fn m2_py_skill_get_type(skill_id: u32) -> u8 {
    get_bridge().get_skill_type(skill_id)
}

#[no_mangle]
pub extern "C" fn m2_py_skill_get_max_level(skill_id: u32) -> u8 {
    get_bridge().get_skill_max_level(skill_id)
}

#[no_mangle]
pub extern "C" fn m2_py_skill_get_grade_name(grade_id: u8) -> *const c_char {
    get_bridge().get_skill_grade_name(grade_id)
}

#[no_mangle]
pub extern "C" fn m2_py_skill_get_description(skill_id: u32) -> *const c_char {
    get_bridge().get_skill_description(skill_id)
}

#[no_mangle]
pub extern "C" fn m2_py_skill_can_use_skill(skill_id: u32) -> bool {
    crate::py_bridge_ui_subsystems::m2_py_ui_skill_can_use_skill(skill_id)
}

#[no_mangle]
pub extern "C" fn m2_py_skill_get_cool_time(skill_id: u32) -> f32 {
    crate::skill_data_cache_v2::m2_skill_get_cooldown_pct(skill_id, crate::app_timing_clock::m2_get_time() as u32)
}

#[no_mangle]
pub extern "C" fn m2_py_skill_get_level(skill_id: u32) -> u32 {
    crate::py_bridge_player_skills::m2_py_player_get_skill_level(skill_id)
}

