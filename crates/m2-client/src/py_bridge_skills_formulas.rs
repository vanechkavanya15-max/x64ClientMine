// crates/m2-client/src/py_bridge_skills_formulas.rs
//! Mostek odczytu wymogow umiejetnosci, formul bonusow i zegarow zadan (quest).
//! Zapewnia 100% parytetu z C++ PythonSkill.cpp i PythonQuest.cpp.

use std::collections::HashMap;
use std::ffi::{c_char, CStr, CString};
use std::sync::RwLock;
use std::time::{SystemTime, UNIX_EPOCH};
use once_cell::sync::Lazy;
use m2_client_bridge::ffi_guard::safe_ffi_call;
use crate::cpoly_evaluator::CPoly;

#[derive(Debug, Clone, Default)]
pub struct SkillRequireStat {
    pub point_type: u8,
    pub level_val: u8,
}

#[derive(Debug, Clone, Default)]
pub struct SkillNewAffect {
    pub point_type_str: String,
    pub formula: String,
}

#[derive(Debug, Clone, Default)]
pub struct SkillDataRecord {
    pub require_stats: Vec<SkillRequireStat>,
    pub new_affects: Vec<SkillNewAffect>,
}

#[derive(Debug, Clone, Default)]
pub struct QuestRecord {
    pub clock_name: String,
    pub start_time: i32,
    pub clock_value: i32,
}

static SKILL_DATABASE: Lazy<RwLock<HashMap<u32, SkillDataRecord>>> = Lazy::new(|| {
    let mut map = HashMap::new();
    // Domyslne rejestracje dla podstawowych skilli (np. Aura Miecza 1, Berserk 2, Wirek 3, Trzystopniowe 4)
    let mut aura = SkillDataRecord::default();
    aura.require_stats.push(SkillRequireStat { point_type: 1, level_val: 5 }); // Wymog sily
    aura.new_affects.push(SkillNewAffect {
        point_type_str: "ATT_GRADE_BONUS".to_string(),
        formula: "50 + 2 * k".to_string(),
    });
    map.insert(1, aura);

    let mut berserk = SkillDataRecord::default();
    berserk.require_stats.push(SkillRequireStat { point_type: 2, level_val: 5 });
    berserk.new_affects.push(SkillNewAffect {
        point_type_str: "ATT_SPEED_BONUS".to_string(),
        formula: "10 + 1.5 * k".to_string(),
    });
    map.insert(2, berserk);

    RwLock::new(map)
});

static QUEST_DATABASE: Lazy<RwLock<HashMap<u32, QuestRecord>>> = Lazy::new(|| RwLock::new(HashMap::new()));

/// Rejestruje dane testowe/konfiguracyjne skilla.
pub fn register_skill_formula_data(skill_index: u32, data: SkillDataRecord) {
    let mut map = SKILL_DATABASE.write().unwrap();
    map.insert(skill_index, data);
}

/// Odczytuje wymogi atrybutow postaci dla danego skilla (skill.GetSkillRequireStatData).
/// Zwraca (byPoint, byLevel).
#[no_mangle]
pub extern "C" fn m2_skill_get_require_stat_data(
    skill_index: u32,
    stat_index: u32,
    out_point: *mut u8,
    out_level: *mut u8,
) -> bool {
    safe_ffi_call(false, || {
        let map = SKILL_DATABASE.read().unwrap();
        if let Some(skill) = map.get(&skill_index) {
            if let Some(req) = skill.require_stats.get(stat_index as usize) {
                unsafe {
                    if !out_point.is_null() { *out_point = req.point_type; }
                    if !out_level.is_null() { *out_level = req.level_val; }
                }
                return true;
            }
        }
        unsafe {
            if !out_point.is_null() { *out_point = 0; }
            if !out_level.is_null() { *out_level = 0; }
        }
        true
    })
}

/// Zwraca liczbe nowych struktur affect dla skilla.
#[no_mangle]
pub extern "C" fn m2_skill_get_new_affect_data_count(skill_index: u32) -> i32 {
    safe_ffi_call(0, || {
        let map = SKILL_DATABASE.read().unwrap();
        map.get(&skill_index).map(|s| s.new_affects.len() as i32).unwrap_or(0)
    })
}

/// Oblicza wartosci min i max bonusu skilla dla danego poziomu (skill.GetNewAffectData).
#[no_mangle]
pub unsafe extern "C" fn m2_skill_get_new_affect_data(
    skill_index: u32,
    affect_index: u32,
    skill_level: f32,
    out_type_buf: *mut c_char,
    out_type_len: usize,
    out_min_val: *mut f32,
    out_max_val: *mut f32,
) -> bool {
    safe_ffi_call(false, || {
        let map = SKILL_DATABASE.read().unwrap();
        let skill = match map.get(&skill_index) {
            Some(s) => s,
            None => return false,
        };
        let affect = match skill.new_affects.get(affect_index as usize) {
            Some(a) => a,
            None => return false,
        };

        if !out_type_buf.is_null() && out_type_len > 0 {
            let c_type = CString::new(affect.point_type_str.clone()).unwrap_or_default();
            let bytes = c_type.as_bytes_with_nul();
            let copy_len = bytes.len().min(out_type_len);
            std::ptr::copy_nonoverlapping(bytes.as_ptr(), out_type_buf as *mut u8, copy_len);
        }

        let mut poly = CPoly::new();
        poly.set_str(&affect.formula);
        poly.set_var("k", skill_level as f64);
        let eval_min = poly.eval() as f32;

        if !out_min_val.is_null() { *out_min_val = eval_min; }
        if !out_max_val.is_null() { *out_max_val = eval_min; }
        true
    })
}

/// Ustawia zegar odliczania questu (quest.SetClock).
#[no_mangle]
pub unsafe extern "C" fn m2_quest_set_clock(
    quest_index: u32,
    clock_name: *const c_char,
    start_time: i32,
    clock_value: i32,
) -> bool {
    safe_ffi_call(false, || {
        let name = if !clock_name.is_null() {
            CStr::from_ptr(clock_name).to_str().unwrap_or("").to_string()
        } else {
            String::new()
        };
        let mut map = QUEST_DATABASE.write().unwrap();
        map.insert(quest_index, QuestRecord {
            clock_name: name,
            start_time,
            clock_value,
        });
        true
    })
}

/// Pobiera nazwe zegara i pozostaly czas trwania misji (quest.GetQuestLastTime).
#[no_mangle]
pub unsafe extern "C" fn m2_quest_get_last_time(
    quest_index: u32,
    out_clock_name: *mut c_char,
    out_name_len: usize,
    out_last_time: *mut i32,
) -> bool {
    safe_ffi_call(false, || {
        let map = QUEST_DATABASE.read().unwrap();
        if let Some(quest) = map.get(&quest_index) {
            if !out_clock_name.is_null() && out_name_len > 0 {
                let c_name = CString::new(quest.clock_name.clone()).unwrap_or_default();
                let bytes = c_name.as_bytes_with_nul();
                let copy_len = bytes.len().min(out_name_len);
                std::ptr::copy_nonoverlapping(bytes.as_ptr(), out_clock_name as *mut u8, copy_len);
            }

            let cur_sec = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|d| d.as_secs() as i32)
                .unwrap_or(0);

            let last_time = if quest.clock_value >= 0 {
                (quest.start_time + quest.clock_value) - cur_sec
            } else {
                0
            };

            if !out_last_time.is_null() {
                *out_last_time = last_time.max(0);
            }
            true
        } else {
            false
        }
    })
}
