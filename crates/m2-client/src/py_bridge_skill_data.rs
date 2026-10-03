use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

// C++ definitions matching SourceCodeClient
const SKILL_ATTRIBUTE_TOGGLE: u32 = 1 << 1;
const SKILL_ATTRIBUTE_STANDING_SKILL: u32 = 1 << 6;

#[derive(Debug, Clone)]
pub struct SkillDataV3 {
    pub skill_id: u32,
    pub skill_type: u8,
    pub level_up_point: u8,
    pub max_level: u8,
    pub skill_attribute: u32,
    pub sp_cost: [u32; 41], // Simplified version of ProcessFormula for NeedSP
}

impl Default for SkillDataV3 {
    fn default() -> Self {
        Self {
            skill_id: 0,
            skill_type: 0,
            level_up_point: 0,
            max_level: 40,
            skill_attribute: 0,
            sp_cost: [0; 41],
        }
    }
}

pub struct PySkillDataBridge {
    skills: HashMap<u32, SkillDataV3>,
}

impl PySkillDataBridge {
    pub fn new() -> Self {
        Self {
            skills: HashMap::new(),
        }
    }

    pub fn register_skill(&mut self, data: SkillDataV3) {
        self.skills.insert(data.skill_id, data);
    }

    pub fn get_skill(&self, skill_id: u32) -> Option<&SkillDataV3> {
        self.skills.get(&skill_id)
    }
}

static PY_SKILL_DATA_BRIDGE: OnceLock<Mutex<PySkillDataBridge>> = OnceLock::new();

pub fn get_bridge() -> std::sync::MutexGuard<'static, PySkillDataBridge> {
    PY_SKILL_DATA_BRIDGE
        .get_or_init(|| Mutex::new(PySkillDataBridge::new()))
        .lock()
        .unwrap()
}

// C-FFI API
#[no_mangle]
pub extern "C" fn m2_py_skill_get_skill_need_sp(skill_index: u32, skill_grade: f32) -> u32 {
    let bridge = get_bridge();
    if let Some(skill) = bridge.get_skill(skill_index) {
        let level = (skill_grade as usize).min(40);
        return skill.sp_cost[level];
    }
    0
}

#[no_mangle]
pub extern "C" fn m2_py_skill_get_skill_level_up_point(skill_index: u32) -> u32 {
    let bridge = get_bridge();
    if let Some(skill) = bridge.get_skill(skill_index) {
        return skill.level_up_point as u32;
    }
    0
}

#[no_mangle]
pub extern "C" fn m2_py_skill_is_toggle_skill(skill_index: u32) -> bool {
    let bridge = get_bridge();
    if let Some(skill) = bridge.get_skill(skill_index) {
        return (skill.skill_attribute & SKILL_ATTRIBUTE_TOGGLE) != 0;
    }
    false
}

#[no_mangle]
pub extern "C" fn m2_py_skill_is_standing_skill(skill_index: u32) -> bool {
    let bridge = get_bridge();
    if let Some(skill) = bridge.get_skill(skill_index) {
        return (skill.skill_attribute & SKILL_ATTRIBUTE_STANDING_SKILL) != 0;
    }
    false
}

#[no_mangle]
pub extern "C" fn m2_py_skill_get_skill_type(skill_index: u32) -> u8 {
    let bridge = get_bridge();
    if let Some(skill) = bridge.get_skill(skill_index) {
        return skill.skill_type;
    }
    0
}
