use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::sync::{Mutex, OnceLock};

use crate::cpoly_evaluator::CPoly;

#[derive(Debug, Clone)]
pub struct AffectData {
    pub description: String,
    pub min_formula: String,
    pub max_formula: String,
}

#[derive(Debug, Clone)]
pub struct SkillDataTableEntry {
    pub skill_vnum: u32,
    pub skill_type: u8,
    pub name: String,
    pub description: String,
    pub affects: Vec<AffectData>,
    pub skill_attribute: u32,
    pub need_weapon: u32,
}

pub struct SkillDataTable {
    skills: HashMap<u32, SkillDataTableEntry>,
    c_string_cache: HashMap<String, CString>,
}

impl SkillDataTable {
    pub fn new() -> Self {
        Self {
            skills: HashMap::new(),
            c_string_cache: HashMap::new(),
        }
    }

    pub fn register_skill(&mut self, entry: SkillDataTableEntry) {
        self.skills.insert(entry.skill_vnum, entry);
    }

    pub fn get_skill(&self, skill_index: u32) -> Option<&SkillDataTableEntry> {
        self.skills.get(&skill_index)
    }

    fn get_c_str(&mut self, s: &str) -> *const c_char {
        if !self.c_string_cache.contains_key(s) {
            self.c_string_cache.insert(s.to_string(), CString::new(s).unwrap_or_default());
        }
        self.c_string_cache.get(s).unwrap().as_ptr()
    }
}

static SKILL_DATA_TABLE: OnceLock<Mutex<SkillDataTable>> = OnceLock::new();

pub fn get_table() -> std::sync::MutexGuard<'static, SkillDataTable> {
    SKILL_DATA_TABLE
        .get_or_init(|| Mutex::new(SkillDataTable::new()))
        .lock()
        .unwrap()
}

// FORMAT_SPECIFIERS from C++
static FORMAT_SPECIFIERS: [&str; 4] = ["%.0f", "%.1f", "%.2f", "%d"];

fn replace_next_format_specifier(s: &mut String, value: f64) -> bool {
    let mut best_pos = None;
    let mut best_len = 0;
    let mut best_spec = "";

    for spec in FORMAT_SPECIFIERS.iter() {
        if let Some(pos) = s.find(*spec) {
            if best_pos.is_none() || pos < best_pos.unwrap() {
                best_pos = Some(pos);
                best_len = spec.len();
                best_spec = spec;
            }
        }
    }

    if let Some(pos) = best_pos {
        let value_str = match best_spec {
            "%.0f" | "%d" => format!("{:.0}", value.floor()),
            "%.1f" => format!("{:.1}", value),
            "%.2f" | _ => format!("{:.2}", value),
        };
        s.replace_range(pos..(pos + best_len), &value_str);
        true
    } else {
        false
    }
}

fn unescape_percent(s: &mut String) {
    let mut pos = 0;
    while let Some(idx) = s[pos..].find("%%") {
        let real_idx = pos + idx;
        s.replace_range(real_idx..(real_idx + 2), "%");
        pos = real_idx + 1;
    }
}

fn get_affect_description(affect: &AffectData, skill_level: f64) -> String {
    let mut min_poly = CPoly::new();
    min_poly.set_str(&affect.min_formula);
    min_poly.set_var("k", skill_level);
    let mut f_min_value = min_poly.eval();
    
    let mut max_poly = CPoly::new();
    max_poly.set_str(&affect.max_formula);
    max_poly.set_var("k", skill_level);
    let mut f_max_value = max_poly.eval();

    if f_min_value < 0.0 { f_min_value = -f_min_value; }
    if f_max_value < 0.0 { f_max_value = -f_max_value; }

    let mut out = affect.description.clone();
    replace_next_format_specifier(&mut out, f_min_value);
    replace_next_format_specifier(&mut out, f_max_value);
    unescape_percent(&mut out);

    out
}

#[no_mangle]
pub extern "C" fn m2_py_skill_get_skill_name(skill_index: u32) -> *const c_char {
    let mut table = get_table();
    if let Some(skill) = table.skills.get(&skill_index) {
        let name = skill.name.clone();
        table.get_c_str(&name)
    } else {
        std::ptr::null()
    }
}

#[no_mangle]
pub extern "C" fn m2_py_skill_get_skill_description(skill_index: u32) -> *const c_char {
    let mut table = get_table();
    if let Some(skill) = table.skills.get(&skill_index) {
        let desc = skill.description.clone();
        table.get_c_str(&desc)
    } else {
        std::ptr::null()
    }
}

#[no_mangle]
pub extern "C" fn m2_py_skill_get_skill_type_v2(skill_index: u32) -> u8 {
    let table = get_table();
    if let Some(skill) = table.get_skill(skill_index) {
        skill.skill_type
    } else {
        0
    }
}

#[no_mangle]
pub extern "C" fn m2_py_skill_get_skill_affect_description(skill_index: u32, affect_index: u32, skill_level: f32) -> *const c_char {
    let mut table = get_table();
    if let Some(skill) = table.skills.get(&skill_index) {
        if affect_index as usize >= skill.affects.len() {
            return std::ptr::null();
        }
        let affect = &skill.affects[affect_index as usize];
        let desc = get_affect_description(affect, skill_level as f64);
        table.get_c_str(&desc)
    } else {
        std::ptr::null()
    }
}

#[no_mangle]
pub extern "C" fn m2_py_skill_get_skill_affect_power(skill_index: u32, affect_index: u32, skill_level: f32, is_max: bool) -> f32 {
    let table = get_table();
    if let Some(skill) = table.skills.get(&skill_index) {
        if affect_index as usize >= skill.affects.len() {
            return 0.0;
        }
        let affect = &skill.affects[affect_index as usize];
        
        let mut poly = CPoly::new();
        if is_max {
            poly.set_str(&affect.max_formula);
        } else {
            poly.set_str(&affect.min_formula);
        }
        poly.set_var("k", skill_level as f64);
        poly.eval() as f32
    } else {
        0.0
    }
}

#[no_mangle]
pub extern "C" fn m2_py_skill_can_use_skill_v2(skill_index: u32) -> bool {
    let table = get_table();
    if let Some(skill) = table.skills.get(&skill_index) {
        // CPythonSkill::SSkillData::IsCanUseSkill
        // SKILL_ATTRIBUTE_PASSIVE is 1 << 17
        let passive = 1 << 17;
        if (skill.skill_attribute & passive) != 0 {
            return false;
        }
        return true;
    }
    false
}

#[no_mangle]
pub extern "C" fn m2_py_skill_can_use_weapon(skill_index: u32, weapon_type: u32) -> bool {
    let table = get_table();
    if let Some(skill) = table.skills.get(&skill_index) {
        // CPythonSkill::SSkillData::CanUseWeaponType
        // SKILL_ATTRIBUTE_WEAPON_LIMITATION = (1 << 2)
        let weapon_limitation = 1 << 2;
        if (skill.skill_attribute & weapon_limitation) == 0 {
            return true;
        }
        return (skill.need_weapon & (1 << weapon_type)) != 0;
    }
    false
}
