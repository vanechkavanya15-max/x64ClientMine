//! Modul `player_skill` — Wykonywanie umiejetnosci bojowych i odliczanie cooldownow
//! Wzorzec referencyjny C++: `source/UserInterface/PythonPlayerSkill.cpp`
//! Opis: Odwzorowanie UseSkill: sprawdzanie wymaganego SP, odliczanie czasu odnowienia (cooldown), obsluga skilli przelaczanych (Aura Miecza, Czarowane Ostrze, Silne Cialo).

pub const SKILL_MAX_NUM: usize = 255;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PointType {
    None = 0,
    Exp = 3,
    NextExp = 4,
    Hp = 5,
    MaxHp = 6,
    Sp = 7,
    MaxSp = 8,
    Stamina = 9,
    MaxStamina = 10,
    Gold = 11,
    St = 12,
    Ht = 13,
    Dx = 14,
    Iq = 15,
    AttPower = 16,
    AttSpeed = 17,
    EvadeRate = 18,
    MovSpeed = 19,
    DefGrade = 20,
    CastingSpeed = 21,
    MagicAttGrade = 22,
    MagicDefGrade = 23,
    BowDistance = 34,
}

#[derive(Debug, Clone, Default)]
pub struct SkillInstance {
    pub index: u32,
    pub skill_type: i32,
    pub grade: i32,
    pub level: i32,
    pub cur_efficient_percentage: f32,
    pub next_efficient_percentage: f32,
    pub is_cool_time: bool,
    pub cool_time: f32,
    pub last_used_time: f32,
    pub active: bool,
}

#[derive(Debug, Clone)]
pub struct PlayerSkill {
    pub initialized: bool,
    pub skills: Vec<SkillInstance>,
}

impl Default for PlayerSkill {
    fn default() -> Self {
        Self {
            initialized: true,
            skills: vec![SkillInstance::default(); SKILL_MAX_NUM],
        }
    }
}

impl PlayerSkill {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }
}


pub trait SkillData {
    fn get_type(&self) -> u8;
    fn is_toggle_skill(&self) -> bool;
    fn is_can_use_skill(&self) -> bool;
    fn is_standing_skill(&self) -> bool;
    fn is_attack_skill(&self) -> bool;
    fn is_need_empty_bottle(&self) -> bool;
    fn is_need_poison_bottle(&self) -> bool;
    fn is_horse_skill(&self) -> bool;
    fn is_charge_skill(&self) -> bool;
    fn is_need_bow(&self) -> bool;
    fn is_use_hp_skill(&self) -> bool;
    fn get_need_sp(&self, cur_efficient_percentage: f32) -> i32;
    fn can_use_if_not_enough(&self) -> bool;
    fn can_use_weapon_type(&self, weapon_type: i32) -> bool;
    fn get_skill_cool_time(&self, cur_efficient_percentage: f32) -> f32;
    fn get_target_range(&self) -> i32;
    fn get_target_count(&self, cur_efficient_percentage: f32) -> i32;
    fn is_fan_range(&self) -> bool;
    fn is_circle_range(&self) -> bool;
    fn is_no_motion(&self) -> bool;
    fn get_skill_motion_index(&self, grade: i32) -> i32;
    fn get_motion_loop_count(&self, cur_efficient_percentage: f32) -> i32;
    fn is_moving_skill(&self) -> bool;
    fn is_only_for_guild_war(&self) -> bool;
}

pub trait PlayerState {
    fn get_status(&self, point_type: PointType) -> i32;
    fn is_fishing_mode(&self) -> bool;
    fn get_weapon_type(&self) -> i32;
    fn has_item(&self, item_vnum: u32) -> bool;
    fn has_enough_arrow(&self) -> bool;
    fn is_in_safe(&self) -> bool;
    fn is_mounting_horse(&self) -> bool;
    fn check_dash_affect(&self) -> bool;
    fn get_current_second(&self) -> f32;
}

impl PlayerSkill {
    pub fn check_short_mana(
        &self,
        slot_index: usize,
        skill_data: &impl SkillData,
        player_state: &impl PlayerState,
    ) -> bool {
        let skill_inst = &self.skills[slot_index];
        
        // Zignoruj skill 161 (c_iSkillIndex_Summon) tak jak w C++
        if skill_inst.index == 161 {
            return false;
        }

        let need_sp = skill_data.get_need_sp(skill_inst.cur_efficient_percentage);
        let cur_sp = player_state.get_status(PointType::Sp);

        if !skill_data.is_toggle_skill() {
            if need_sp == 0 {
                // To wywoluje blad, wiec zwracamy prawde jako short mana (zabezpieczenie gry)
                return true;
            }
        }

        if skill_data.can_use_if_not_enough() {
            if cur_sp <= 0 {
                return true;
            }
        } else {
            if need_sp != -1 {
                if need_sp > cur_sp {
                    return true;
                }
            }
        }

        false
    }

    pub fn check_short_life(
        &self,
        slot_index: usize,
        skill_data: &impl SkillData,
        player_state: &impl PlayerState,
    ) -> bool {
        let skill_inst = &self.skills[slot_index];
        if !skill_data.is_use_hp_skill() {
            return false;
        }

        let need_hp = skill_data.get_need_sp(skill_inst.cur_efficient_percentage) as i32;
        if need_hp <= player_state.get_status(PointType::Hp) {
            return false;
        }

        true
    }

    pub fn check_short_arrow(
        &self,
        skill_data: &impl SkillData,
        player_state: &impl PlayerState,
    ) -> bool {
        if !skill_data.is_need_bow() {
            return false;
        }

        if player_state.has_enough_arrow() {
            return false;
        }

        true
    }

    pub fn check_rest_skill_cool_time(
        &self,
        slot_index: usize,
        player_state: &impl PlayerState,
    ) -> bool {
        if slot_index >= SKILL_MAX_NUM {
            return false;
        }

        let skill_inst = &self.skills[slot_index];
        let elapsed_time = player_state.get_current_second() - skill_inst.last_used_time;
        
        if elapsed_time >= skill_inst.cool_time {
            return false;
        }

        true
    }

    pub fn check_skill_usable(
        &self,
        slot_index: usize,
        skill_data: &impl SkillData,
        player_state: &impl PlayerState,
        is_level_limit: bool,
    ) -> bool {
        if slot_index >= SKILL_MAX_NUM {
            return false;
        }

        let skill_inst = &self.skills[slot_index];

        if player_state.is_mounting_horse() {
            if !skill_data.is_horse_skill() {
                return false;
            }
        }

        if skill_data.is_horse_skill() {
            if !player_state.is_mounting_horse() {
                return false;
            }
        }

        if skill_data.is_attack_skill() {
            if player_state.is_in_safe() {
                return false;
            }
        }

        if !skill_data.is_can_use_skill() {
            return false;
        }

        if skill_data.is_need_empty_bottle() {
            if !player_state.has_item(27995) {
                return false;
            }
        }

        if skill_data.is_need_poison_bottle() {
            if !player_state.has_item(27996) {
                return false;
            }
        }

        if player_state.is_fishing_mode() {
            return false;
        }

        if is_level_limit {
            if skill_inst.level <= 0 {
                return false;
            }
        }

        if !skill_data.can_use_weapon_type(player_state.get_weapon_type()) {
            return false;
        }

        if !skill_data.is_horse_skill() {
            if self.check_short_arrow(skill_data, player_state) {
                return false;
            }

            if skill_data.is_need_bow() {
                if !player_state.has_enough_arrow() {
                    return false;
                }
            }
        }

        if player_state.check_dash_affect() {
            if !skill_data.is_charge_skill() {
                if self.check_rest_skill_cool_time(slot_index, player_state) {
                    return false;
                }
            }
        } else {
            if self.check_rest_skill_cool_time(slot_index, player_state) {
                return false;
            }

            if self.check_short_life(slot_index, skill_data, player_state) {
                return false;
            }

            if self.check_short_mana(slot_index, skill_data, player_state) {
                return false;
            }
        }

        true
    }
}

impl PlayerSkill {
    pub fn run_cool_time(
        &mut self,
        slot_index: usize,
        skill_data: &impl SkillData,
        player_state: &impl PlayerState,
    ) {
        if slot_index >= SKILL_MAX_NUM {
            return;
        }

        let cur_percentage = self.skills[slot_index].cur_efficient_percentage;
        let mut cool_time = skill_data.get_skill_cool_time(cur_percentage);
        self.skills[slot_index].last_used_time = player_state.get_current_second();

        let mut spd = 100 - player_state.get_status(PointType::CastingSpeed);
        if spd > 0 {
            spd = 100 + spd;
        } else if spd < 0 {
            spd = 10000 / (100 - spd);
        } else {
            spd = 100;
        }

        cool_time = cool_time * (spd as f32) / 100.0;
        self.skills[slot_index].cool_time = cool_time;
    }

    pub fn use_skill(
        &mut self,
        slot_index: usize,
        skill_data: &impl SkillData,
        player_state: &impl PlayerState,
        is_level_limit: bool,
    ) -> bool {
        if slot_index >= SKILL_MAX_NUM {
            return false;
        }

        if skill_data.is_toggle_skill() && self.is_skill_active(slot_index) {
            self.set_skill_active(slot_index, false);
            return false;
        }

        if !self.check_skill_usable(slot_index, skill_data, player_state, is_level_limit) {
            return false;
        }

        if skill_data.is_toggle_skill() {
            self.set_skill_active(slot_index, true);
        }

        self.run_cool_time(slot_index, skill_data, player_state);
        
        true
    }
}

impl PlayerSkill {
    pub fn is_skill_active(&self, slot_index: usize) -> bool {
        if slot_index >= SKILL_MAX_NUM {
            return false;
        }
        self.skills[slot_index].active
    }

    pub fn set_skill_active(&mut self, slot_index: usize, active: bool) {
        if slot_index < SKILL_MAX_NUM {
            self.skills[slot_index].active = active;
        }
    }
    
    // Toggle skill simulation directly inside `use_skill`
    // which aligns with __UseSkill / ClickSkillSlot from PythonPlayer.cpp
}
