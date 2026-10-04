<<<<<<< SEARCH
#[derive(Debug, Clone)]
pub struct SkillDataTableEntry {
    pub skill_vnum: u32,
    pub skill_type: u8,
    pub name: String,
    pub description: String,
    pub affects: Vec<AffectData>,
    pub skill_attribute: u32,
}
=======
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
>>>>>>> REPLACE
<<<<<<< SEARCH
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
=======
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
pub extern "C" fn m2_py_skill_can_use_weapon_v2(skill_index: u32, weapon_type: u32) -> bool {
    let table = get_table();
    if let Some(skill) = table.skills.get(&skill_index) {
        if skill.need_weapon == 0 {
            return true;
        }
        return (skill.need_weapon & (1 << weapon_type)) != 0;
    }
    false
}
>>>>>>> REPLACE
