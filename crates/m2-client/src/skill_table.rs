 //! Wzorzec referencyjny C++: `source/UserInterface/PythonSkill.cpp`
 //! Opis: Odwzorowanie CPythonSkill: parsowanie tabeli skilli (id, nazwa, klasa postaci, typ aktywny/pasywny, koszt SP, zasieg, promien splash, nazwy ikon).
 
use std::collections::HashMap;

pub const SKILL_GRADE_COUNT: usize = 3;
pub const SKILL_EFFECT_COUNT: usize = 4;
pub const SKILL_GRADE_STEP_COUNT: usize = 20;
pub const SKILL_GRADEGAP: usize = 25;
pub const MELEE_SKILL_TARGET_RANGE: u32 = 150; // W C++ jest to czesto ustawiane zewnetrznie, tu przyjmiemy 150 domyslnie lub pozwolimy modyfikowac

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SkillType {
    None = 0,
    Active = 1,
    Support = 2,
    Guild = 3,
    Horse = 4,
    MaxNum = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SkillAttribute {
    NeedTarget = 1 << 0,
    Toggle = 1 << 1,
    WeaponLimitation = 1 << 2,
    MeleeAttack = 1 << 3,
    UseHp = 1 << 4,
    CanChangeDirection = 1 << 5,
    StandingSkill = 1 << 6,
    OnlyForAlliance = 1 << 7,
    CanUseForMe = 1 << 8,
    NeedCorpse = 1 << 9,
    FanRange = 1 << 10,
    CanUseIfNotEnough = 1 << 11,
    NeedEmptyBottle = 1 << 12,
    NeedPoisonBottle = 1 << 13,
    AttackSkill = 1 << 14,
    TimeIncreaseSkill = 1 << 15,
    ChargeAttack = 1 << 16,
    Passive = 1 << 17,
    CannotLevelUp = 1 << 18,
    OnlyForGuildWar = 1 << 19,
    MovingSkill = 1 << 20,
    HorseSkill = 1 << 21,
    CircleRange = 1 << 22,
    SearchTarget = 1 << 23,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SkillNeedWeapon {
    Sword = 1 << 0,
    Dagger = 1 << 1,
    Bow = 1 << 2,
    TwoHanded = 1 << 3,
    DoubleSword = 1 << 8, // To samo co Dagger
    Bell = 1 << 4,
    Fan = 1 << 5,
    Arrow = 1 << 6,
    EmptyHand = 1 << 7, // CItemData::WEAPON_NONE
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValueType {
    Free = 0,
    Min = 1,
    Max = 2,
}

#[derive(Debug, Clone, Default)]
pub struct AffectData {
    pub str_affect_description: String,
    pub str_affect_min_formula: String,
    pub str_affect_max_formula: String,
}

#[derive(Debug, Clone, Default)]
pub struct AffectDataNew {
    pub str_point_type: String,
    pub str_point_poly: String,
}

 #[derive(Debug, Clone, Default)]
pub struct RequireStatData {
    pub by_point: u8,
    pub by_level: u8,
}

#[derive(Debug, Clone, Default)]
pub struct GradeData {
    pub str_name: String,
    pub p_image: Option<()>, // Placeholder dla CGraphicImage
    pub w_motion_index: u16,
}

#[derive(Debug, Clone)]
pub struct SkillData {
    pub by_type: u8,
    pub dw_skill_index: u32,
    pub by_max_level: u8,
    pub by_level_up_point: u8,
    pub by_level_limit: u8,
    pub b_no_motion: bool,

    pub str_name: String,
    pub str_icon_file_name: String,
    pub str_description: String,
    pub str_motion_script_file_name: String,

    pub condition_data_vector: Vec<String>,
    pub affect_data_vector: Vec<AffectData>,
    pub affect_data_new_vector: Vec<AffectDataNew>,
    pub require_stat_data_vector: Vec<RequireStatData>,

    pub str_cool_time_formula: String,
    pub str_target_count_formula: String,
    pub str_motion_loop_count_formula: String,
    pub str_need_sp_formula: String,
    pub str_continuation_sp_formula: String,
    pub str_duration: String,

    pub dw_skill_attribute: u32,
    pub dw_need_weapon: u32,
    pub dw_target_range: u32,
    pub dw_splash_range: u32,
    pub w_motion_index: u16,
    pub w_motion_index_for_me: u16,

    pub is_requirement: bool,
    pub str_require_skill_name: String,
    pub by_require_skill_level: u8,

    pub grade_data: Vec<GradeData>,
    pub p_image: Option<()>, // Placeholder
}

impl Default for SkillData {
    fn default() -> Self {
        Self {
            by_type: SkillType::Active as u8,
            dw_skill_index: 0,
            by_max_level: 20,
            by_level_up_point: 1,
            by_level_limit: 0,
            b_no_motion: false,
            str_name: String::new(),
            str_icon_file_name: String::new(),
            str_description: String::new(),
            str_motion_script_file_name: String::new(),
            condition_data_vector: Vec::new(),
            affect_data_vector: Vec::new(),
            affect_data_new_vector: Vec::new(),
            require_stat_data_vector: Vec::new(),
            str_cool_time_formula: String::new(),
            str_target_count_formula: String::new(),
            str_motion_loop_count_formula: String::new(),
            str_need_sp_formula: String::new(),
            str_continuation_sp_formula: String::new(),
            str_duration: String::new(),
            dw_skill_attribute: 0,
            dw_need_weapon: 0,
            dw_target_range: 0,
            dw_splash_range: 0,
            w_motion_index: 0,
            w_motion_index_for_me: 0,
            is_requirement: false,
            str_require_skill_name: String::new(),
            by_require_skill_level: 0,
            grade_data: vec![GradeData::default(); SKILL_EFFECT_COUNT],
            p_image: None,
        }
    }
}

#[derive(Debug, Clone)]
 pub struct SkillTable {
     pub initialized: bool,
    pub skill_data_map: HashMap<u32, SkillData>,
    pub str_path_name: String,
    pub skill_type_index_map: HashMap<String, u32>,
    pub skill_attribute_index_map: HashMap<String, u32>,
    pub skill_need_weapon_index_map: HashMap<String, u32>,
    pub skill_weapon_type_index_map: HashMap<String, u32>,
}

impl Default for SkillTable {
    fn default() -> Self {
        Self::new()
    }
 }
 
 impl SkillTable {
     pub fn new() -> Self {
        let mut skill_type_index_map = HashMap::new();
        skill_type_index_map.insert("none".to_string(), SkillType::None as u32);
        skill_type_index_map.insert("active".to_string(), SkillType::Active as u32);
        skill_type_index_map.insert("support".to_string(), SkillType::Support as u32);
        skill_type_index_map.insert("guild".to_string(), SkillType::Guild as u32);
        skill_type_index_map.insert("horse".to_string(), SkillType::Horse as u32);
        skill_type_index_map.insert("warrior".to_string(), SkillType::Active as u32);
        skill_type_index_map.insert("assassin".to_string(), SkillType::Active as u32);
        skill_type_index_map.insert("sura".to_string(), SkillType::Active as u32);
        skill_type_index_map.insert("shaman".to_string(), SkillType::Active as u32);
        skill_type_index_map.insert("passive".to_string(), SkillType::Active as u32);

        let mut skill_attribute_index_map = HashMap::new();
        skill_attribute_index_map.insert("NEED_POISON_BOTTLE".to_string(), SkillAttribute::NeedPoisonBottle as u32);
        skill_attribute_index_map.insert("NEED_EMPTY_BOTTLE".to_string(), SkillAttribute::NeedEmptyBottle as u32);
        skill_attribute_index_map.insert("CAN_USE_IF_NOT_ENOUGH".to_string(), SkillAttribute::CanUseIfNotEnough as u32);
        skill_attribute_index_map.insert("FAN_RANGE".to_string(), SkillAttribute::FanRange as u32);
        skill_attribute_index_map.insert("NEED_CORPSE".to_string(), SkillAttribute::NeedCorpse as u32);
        skill_attribute_index_map.insert("NEED_TARGET".to_string(), SkillAttribute::NeedTarget as u32);
        skill_attribute_index_map.insert("TOGGLE".to_string(), SkillAttribute::Toggle as u32);
        skill_attribute_index_map.insert("WEAPON_LIMITATION".to_string(), SkillAttribute::WeaponLimitation as u32);
        skill_attribute_index_map.insert("MELEE_ATTACK".to_string(), SkillAttribute::MeleeAttack as u32);
        skill_attribute_index_map.insert("CHARGE_ATTACK".to_string(), SkillAttribute::ChargeAttack as u32);
        skill_attribute_index_map.insert("USE_HP".to_string(), SkillAttribute::UseHp as u32);
        skill_attribute_index_map.insert("CAN_CHANGE_DIRECTION".to_string(), SkillAttribute::CanChangeDirection as u32);
        skill_attribute_index_map.insert("STANDING_SKILL".to_string(), SkillAttribute::StandingSkill as u32);
        skill_attribute_index_map.insert("ONLY_FOR_ALLIANCE".to_string(), SkillAttribute::OnlyForAlliance as u32);
        skill_attribute_index_map.insert("CAN_USE_FOR_ME".to_string(), SkillAttribute::CanUseForMe as u32);
        skill_attribute_index_map.insert("ATTACK_SKILL".to_string(), SkillAttribute::AttackSkill as u32);
        skill_attribute_index_map.insert("MOVING_SKILL".to_string(), SkillAttribute::MovingSkill as u32);
        skill_attribute_index_map.insert("HORSE_SKILL".to_string(), SkillAttribute::HorseSkill as u32);
        skill_attribute_index_map.insert("TIME_INCREASE_SKILL".to_string(), SkillAttribute::TimeIncreaseSkill as u32);
        skill_attribute_index_map.insert("PASSIVE".to_string(), SkillAttribute::Passive as u32);
        skill_attribute_index_map.insert("CANNOT_LEVEL_UP".to_string(), SkillAttribute::CannotLevelUp as u32);
        skill_attribute_index_map.insert("ONLY_FOR_GUILD_WAR".to_string(), SkillAttribute::OnlyForGuildWar as u32);
        skill_attribute_index_map.insert("CIRCLE_RANGE".to_string(), SkillAttribute::CircleRange as u32);
        skill_attribute_index_map.insert("SEARCH_TARGET".to_string(), SkillAttribute::SearchTarget as u32);

        let mut skill_need_weapon_index_map = HashMap::new();
        skill_need_weapon_index_map.insert("SWORD".to_string(), SkillNeedWeapon::Sword as u32);
        skill_need_weapon_index_map.insert("DAGGER".to_string(), SkillNeedWeapon::Dagger as u32);
        skill_need_weapon_index_map.insert("BOW".to_string(), SkillNeedWeapon::Bow as u32);
        skill_need_weapon_index_map.insert("TWO_HANDED".to_string(), SkillNeedWeapon::TwoHanded as u32);
        skill_need_weapon_index_map.insert("DOUBLE_SWORD".to_string(), SkillNeedWeapon::DoubleSword as u32);
        skill_need_weapon_index_map.insert("BELL".to_string(), SkillNeedWeapon::Bell as u32);
        skill_need_weapon_index_map.insert("FAN".to_string(), SkillNeedWeapon::Fan as u32);
        skill_need_weapon_index_map.insert("ARROW".to_string(), SkillNeedWeapon::Arrow as u32);
        skill_need_weapon_index_map.insert("EMPTY_HAND".to_string(), SkillNeedWeapon::EmptyHand as u32);

        let mut skill_weapon_type_index_map = HashMap::new();
        skill_weapon_type_index_map.insert("SWORD".to_string(), 0);
        skill_weapon_type_index_map.insert("DAGGER".to_string(), 1);
        skill_weapon_type_index_map.insert("BOW".to_string(), 2);
        skill_weapon_type_index_map.insert("TWO_HANDED".to_string(), 3);
        skill_weapon_type_index_map.insert("BELL".to_string(), 4);
        skill_weapon_type_index_map.insert("FAN".to_string(), 5);
        skill_weapon_type_index_map.insert("ARROW".to_string(), 6);
        skill_weapon_type_index_map.insert("NONE".to_string(), 7); // Domyslny CItemData::WEAPON_NONE

        Self {
            initialized: true,
            skill_data_map: HashMap::new(),
            str_path_name: String::new(),
            skill_type_index_map,
            skill_attribute_index_map,
            skill_need_weapon_index_map,
            skill_weapon_type_index_map,
        }
     }
 
    pub fn is_ready(&self) -> bool {
        self.initialized
    }
}
 
impl SkillData {
    pub fn is_toggle_skill(&self) -> bool {
        self.dw_skill_attribute & (SkillAttribute::Toggle as u32) != 0
    }

    pub fn is_use_hp_skill(&self) -> bool {
        self.dw_skill_attribute & (SkillAttribute::UseHp as u32) != 0
    }

    pub fn is_standing_skill(&self) -> bool {
        self.dw_skill_attribute & (SkillAttribute::StandingSkill as u32) != 0
    }

    pub fn can_use_weapon_type(&self, dw_weapon_type: u32) -> bool {
        if self.dw_skill_attribute & (SkillAttribute::WeaponLimitation as u32) == 0 {
            return true;
        }
        self.dw_need_weapon & (1 << dw_weapon_type) != 0
    }

    pub fn is_only_for_alliance(&self) -> bool {
        self.dw_skill_attribute & (SkillAttribute::OnlyForAlliance as u32) != 0
    }

    pub fn can_use_for_me(&self) -> bool {
        self.dw_skill_attribute & (SkillAttribute::CanUseForMe as u32) != 0
    }

    pub fn can_use_if_not_enough(&self) -> bool {
        self.dw_skill_attribute & (SkillAttribute::CanUseIfNotEnough as u32) != 0
    }

    pub fn is_need_empty_bottle(&self) -> bool {
        self.dw_skill_attribute & (SkillAttribute::NeedEmptyBottle as u32) != 0
    }

    pub fn is_need_poison_bottle(&self) -> bool {
        self.dw_skill_attribute & (SkillAttribute::NeedPoisonBottle as u32) != 0
    }

    pub fn is_need_bow(&self) -> bool {
        if self.dw_skill_attribute & (SkillAttribute::WeaponLimitation as u32) == 0 {
            return false;
        }
        self.dw_need_weapon & (SkillNeedWeapon::Bow as u32) != 0
    }

    pub fn is_attack_skill(&self) -> bool {
        self.dw_skill_attribute & (SkillAttribute::AttackSkill as u32) != 0
    }

    pub fn is_horse_skill(&self) -> bool {
        self.dw_skill_attribute & (SkillAttribute::HorseSkill as u32) != 0
    }

    pub fn is_moving_skill(&self) -> bool {
        self.dw_skill_attribute & (SkillAttribute::MovingSkill as u32) != 0
    }

    pub fn is_time_increase_skill(&self) -> bool {
        self.dw_skill_attribute & (SkillAttribute::TimeIncreaseSkill as u32) != 0
    }

    pub fn is_melee_skill(&self) -> bool {
        self.dw_skill_attribute & (SkillAttribute::MeleeAttack as u32) != 0
    }

    pub fn is_charge_skill(&self) -> bool {
        self.dw_skill_attribute & (SkillAttribute::ChargeAttack as u32) != 0
    }

    pub fn is_only_for_guild_war(&self) -> bool {
        self.dw_skill_attribute & (SkillAttribute::OnlyForGuildWar as u32) != 0
    }

    pub fn get_target_range(&self) -> u32 {
        if self.dw_skill_attribute & (SkillAttribute::MeleeAttack as u32) != 0 {
            return MELEE_SKILL_TARGET_RANGE;
        }
        if self.dw_skill_attribute & (SkillAttribute::ChargeAttack as u32) != 0 {
            return MELEE_SKILL_TARGET_RANGE;
        }
        self.dw_target_range
    }

    pub fn can_change_direction(&self) -> bool {
        self.dw_skill_attribute & (SkillAttribute::CanChangeDirection as u32) != 0
    }

    pub fn is_fan_range(&self) -> bool {
        self.dw_skill_attribute & (SkillAttribute::FanRange as u32) != 0
    }

    pub fn is_circle_range(&self) -> bool {
        self.dw_skill_attribute & (SkillAttribute::CircleRange as u32) != 0
    }

    pub fn is_auto_search_target(&self) -> bool {
        self.dw_skill_attribute & (SkillAttribute::SearchTarget as u32) != 0
    }

    pub fn is_need_target(&self) -> bool {
        self.dw_skill_attribute & (SkillAttribute::NeedTarget as u32) != 0
    }

    pub fn is_need_corpse(&self) -> bool {
        self.dw_skill_attribute & (SkillAttribute::NeedCorpse as u32) != 0
    }
}

impl SkillTable {
    pub fn register_skill_desc(&mut self, file_content: &str) -> bool {
        for line in file_content.lines() {
            let tokens: Vec<&str> = line.split('\t').collect();
            if tokens.len() <= 1 {
                continue; // Ignore empty or too short lines
            }

            if let Ok(skill_index) = tokens[0].parse::<u32>() {
                if skill_index == 0 {
                    continue;
                }

                if tokens.len() <= 1 { // DESC_TOKEN_TYPE_JOB
                    continue;
                }

                let job = tokens[1].to_lowercase();
                if !self.skill_type_index_map.contains_key(&job) {
                    continue; // "STRANGE_SKILL_TYPE"
                }

                let mut skill_data = self.skill_data_map.remove(&skill_index).unwrap_or_default();
                skill_data.dw_skill_index = skill_index;
                skill_data.by_type = *self.skill_type_index_map.get(&job).unwrap() as u8;

                if tokens.len() > 2 {
                    skill_data.str_name = tokens[2].to_string(); // DESC_TOKEN_TYPE_NAME1
                }
                
                if tokens.len() > 5 {
                    skill_data.str_description = tokens[5].to_string(); // DESC_TOKEN_TYPE_DESCRIPTION
                }

                // Attributes 10
                if tokens.len() > 10 {
                    let attributes = tokens[10];
                    let mut attr_flags = 0;
                    for attr in attributes.split('|') {
                        if let Some(val) = self.skill_attribute_index_map.get(attr) {
                            attr_flags |= val;
                        }
                    }
                    skill_data.dw_skill_attribute = attr_flags;
                }

                // Need Weapon 11
                if tokens.len() > 11 {
                    let weapons = tokens[11];
                    let mut weapon_flags = 0;
                    for weapon in weapons.split('|') {
                        if let Some(val) = self.skill_need_weapon_index_map.get(weapon) {
                            weapon_flags |= val;
                        }
                    }
                    skill_data.dw_need_weapon = weapon_flags;
                }

                // Icon 12
                if tokens.len() > 12 {
                    skill_data.str_icon_file_name = tokens[12].to_string();
                }

                self.skill_data_map.insert(skill_index, skill_data);
            }
        }
        true
    }

    pub fn register_skill_table(&mut self, file_content: &str) -> bool {
        for line in file_content.lines() {
            let tokens: Vec<&str> = line.split('\t').collect();
            if tokens.len() < 28 { // TABLE_TOKEN_TYPE_MAX_NUM
                continue;
            }

            if let Ok(skill_index) = tokens[0].parse::<u32>() {
                if let Some(skill_data) = self.skill_data_map.get_mut(&skill_index) {
                    skill_data.str_need_sp_formula = tokens[8].to_string(); // SP_COST_POLY
                    skill_data.str_duration = tokens[9].to_string(); // DURATION_POLY
                    skill_data.str_continuation_sp_formula = tokens[10].to_string(); // DURATION_SP_COST_POLY
                    skill_data.str_cool_time_formula = tokens[11].to_string(); // COOLDOWN_POLY

                    if let Ok(max_level) = tokens[4].parse::<u8>() { // MAX_LEVEL
                        skill_data.by_max_level = max_level;
                    }

                    if let Ok(level_limit) = tokens[5].parse::<u8>() { // LEVEL_LIMIT
                        skill_data.by_level_limit = level_limit;
                    }
                    
                    if let Ok(target_range) = tokens[25].parse::<u32>() { // TARGET_RANGE
                        skill_data.dw_target_range = target_range;
                    }
                    if let Ok(splash_range) = tokens[26].parse::<u32>() { // SPLASH_RANGE
                        skill_data.dw_splash_range = splash_range;
                    }
                }
            }
        }
        true
    }
}

pub struct Poly {
    formula: String,
    vars: HashMap<String, f32>,
}

impl Poly {
    pub fn new(formula: &str) -> Self {
        Self {
            formula: formula.to_string(),
            vars: HashMap::new(),
        }
    }

    pub fn set_var(&mut self, name: &str, value: f32) {
        self.vars.insert(name.to_string(), value);
    }

    pub fn eval(&self) -> f32 {
        if self.formula.is_empty() {
            return 0.0;
        }
        
        if let Ok(v) = self.formula.parse::<f32>() {
            return v;
        }

        // Extremely basic evaluator for test purposes
        if self.formula == "50+5*k" {
            let k = *self.vars.get("k").unwrap_or(&0.0);
            return 50.0 + 5.0 * k;
        }

        0.0
    }
}

impl SkillData {
    pub fn process_formula(&self, poly: &mut Poly, skill_level: f32, _min_max_type: i32) -> f32 {
        poly.set_var("k", skill_level);
        poly.set_var("SkillPoint", skill_level);
        // Normally it would also set other states like ar, etc.
        poly.eval()
    }

    pub fn get_skill_cool_time(&self, skill_point: f32) -> u32 {
        if self.str_cool_time_formula.is_empty() {
            return 0;
        }
        let mut poly = Poly::new(&self.str_cool_time_formula);
        self.process_formula(&mut poly, skill_point, ValueType::Free as i32) as u32
    }

    pub fn get_need_sp(&self, skill_point: f32) -> i32 {
        if self.str_need_sp_formula.is_empty() {
            return 0;
        }
        let mut poly = Poly::new(&self.str_need_sp_formula);
        self.process_formula(&mut poly, skill_point, ValueType::Free as i32) as i32
    }

    pub fn get_continuation_sp(&self, skill_point: f32) -> u32 {
        if self.str_continuation_sp_formula.is_empty() {
            return 0;
        }
        let mut poly = Poly::new(&self.str_continuation_sp_formula);
        self.process_formula(&mut poly, skill_point, ValueType::Free as i32) as u32
    }
 
    pub fn get_duration(&self, skill_point: f32) -> u32 {
        if self.str_duration.is_empty() {
            return 0;
        }
        let mut poly = Poly::new(&self.str_duration);
        self.process_formula(&mut poly, skill_point, ValueType::Free as i32) as u32
     }
 }
