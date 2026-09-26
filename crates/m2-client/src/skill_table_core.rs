use std::collections::HashMap;

pub const SKILL_TYPE_ATTACK: u8 = 1;
pub const SKILL_TYPE_BUFF: u8 = 2;
pub const SKILL_TYPE_TOGGLE: u8 = 3;

#[derive(Debug, Clone)]
pub struct SkillData {
    pub skill_vnum: u32,
    pub skill_type: u8,
    pub base_sp_cost: u32,
    pub cooldown_base_ms: u32,
    pub max_level: u8,
}

impl Default for SkillData {
    fn default() -> Self {
        Self {
            skill_vnum: 0,
            skill_type: SKILL_TYPE_ATTACK,
            base_sp_cost: 0,
            cooldown_base_ms: 0,
            max_level: 40,
        }
    }
}

pub struct SkillTableCore {
    skills: HashMap<u32, SkillData>,
}

impl SkillTableCore {
    pub fn new() -> Self {
        Self {
            skills: HashMap::new(),
        }
    }

    pub fn register_skill(&mut self, data: SkillData) {
        self.skills.insert(data.skill_vnum, data);
    }

    pub fn get_skill(&self, vnum: u32) -> Option<&SkillData> {
        self.skills.get(&vnum)
    }

    pub fn calculate_sp_cost(&self, vnum: u32, skill_level: u8) -> Option<u32> {
        let skill = self.get_skill(vnum)?;
        if skill_level == 0 {
            return Some(0); // Skill not learned
        }
        
        let level = skill_level.min(skill.max_level);
        
        // Base cost + 10% for each level above 1
        let extra_cost = (skill.base_sp_cost.saturating_mul(level.saturating_sub(1) as u32)) / 10;
        Some(skill.base_sp_cost.saturating_add(extra_cost))
    }

    pub fn can_use_skill(&self, vnum: u32, skill_level: u8, current_sp: u32) -> Result<bool, &'static str> {
        let cost = self.calculate_sp_cost(vnum, skill_level).ok_or("Skill not found")?;
        Ok(current_sp >= cost)
    }
}
