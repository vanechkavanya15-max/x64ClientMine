//! Python Bridge Player Skills Evaluation Module
//!
//! Ewaluacja mozliwosci uzycia umiejetnosci gracza (CanUseSkill), sprawdzanie
//! wymagan punktow many (SP), broni oraz czasu odnowienia (Cooltime) w FFI Pythona.

use std::collections::HashMap;
use std::sync::Mutex;
use lazy_static::lazy_static;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SkillRequirement {
    pub skill_id: u32,
    pub min_level: u32,
    pub need_sp: u32,
    pub cooltime_total: f32,
    pub need_weapon: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PlayerSkillStatus {
    pub skill_id: u32,
    pub level: u32,
    pub last_used_time: f32,
    pub cooltime_remaining: f32,
}

#[derive(Debug, Clone, Default)]
pub struct PlayerSkillEvaluator {
    pub current_sp: u32,
    pub player_level: u32,
    pub has_weapon_equipped: bool,
    pub is_stunned: bool,
    pub requirements: HashMap<u32, SkillRequirement>,
    pub skill_states: HashMap<u32, PlayerSkillStatus>,
}

impl PlayerSkillEvaluator {
    pub fn new() -> Self {
        let mut eval = Self {
            current_sp: 100,
            player_level: 1,
            has_weapon_equipped: true,
            is_stunned: false,
            requirements: HashMap::new(),
            skill_states: HashMap::new(),
        };

        // Przykladowe rejestracje umiejetnosci (np. 1 = Ciecie Trzystopniowe, 4 = Aura Miecza)
        eval.register_skill_req(SkillRequirement {
            skill_id: 1,
            min_level: 5,
            need_sp: 30,
            cooltime_total: 12.0,
            need_weapon: true,
        });

        eval.register_skill_req(SkillRequirement {
            skill_id: 4,
            min_level: 5,
            need_sp: 50,
            cooltime_total: 60.0,
            need_weapon: true,
        });

        eval
    }

    pub fn register_skill_req(&mut self, req: SkillRequirement) {
        self.requirements.insert(req.skill_id, req);
    }

    pub fn can_use_skill(&self, skill_id: u32) -> bool {
        if self.is_stunned {
            return false;
        }

        let req = match self.requirements.get(&skill_id) {
            Some(r) => r,
            None => return false,
        };

        if self.player_level < req.min_level {
            return false;
        }

        if req.need_weapon && !self.has_weapon_equipped {
            return false;
        }

        if self.current_sp < req.need_sp {
            return false;
        }

        if let Some(state) = self.skill_states.get(&skill_id) {
            if state.cooltime_remaining > 0.0 {
                return false;
            }
        }

        true
    }

    pub fn get_skill_need_sp(&self, skill_id: u32) -> u32 {
        self.requirements.get(&skill_id).map(|r| r.need_sp).unwrap_or(0)
    }

    pub fn get_skill_cooltime(&self, skill_id: u32) -> (f32, f32) {
        let total = self.requirements.get(&skill_id).map(|r| r.cooltime_total).unwrap_or(0.0);
        let rem = self.skill_states.get(&skill_id).map(|s| s.cooltime_remaining).unwrap_or(0.0);
        (rem, total)
    }

    pub fn set_skill_used(&mut self, skill_id: u32) {
        if let Some(req) = self.requirements.get(&skill_id) {
            let total = req.cooltime_total;
            let need_sp = req.need_sp;
            self.current_sp = self.current_sp.saturating_sub(need_sp);
            self.skill_states.insert(skill_id, PlayerSkillStatus {
                skill_id,
                level: 1,
                last_used_time: 0.0,
                cooltime_remaining: total,
            });
        }
    }
}

lazy_static! {
    static ref GLOBAL_EVALUATOR: Mutex<PlayerSkillEvaluator> = Mutex::new(PlayerSkillEvaluator::new());
}

#[no_mangle]
pub extern "C" fn player_can_use_skill(skill_id: u32) -> bool {
    GLOBAL_EVALUATOR.lock().map(|e| e.can_use_skill(skill_id)).unwrap_or(false)
}

#[no_mangle]
pub extern "C" fn player_get_skill_need_sp(skill_id: u32) -> u32 {
    GLOBAL_EVALUATOR.lock().map(|e| e.get_skill_need_sp(skill_id)).unwrap_or(0)
}

#[no_mangle]
pub extern "C" fn player_get_skill_cooltime_remaining(skill_id: u32) -> f32 {
    GLOBAL_EVALUATOR.lock().map(|e| e.get_skill_cooltime(skill_id).0).unwrap_or(0.0)
}
