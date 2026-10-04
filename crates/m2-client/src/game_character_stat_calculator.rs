//! Character statistics calculator: VIT, INT, STR, DEX attributes, HP/SP formulas, and defense/attack power.

use std::sync::{Mutex, OnceLock};
use m2_client_bridge::ffi_guard::safe_ffi_call;

pub const STAT_MAX_LIMIT: u16 = 90;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatType {
    Vit = 0,
    Int = 1,
    Str = 2,
    Dex = 3,
}

#[derive(Debug, Clone)]
pub struct CharacterStatsState {
    pub level: u32,
    pub vit: u16,
    pub int_: u16,
    pub str_: u16,
    pub dex: u16,
    pub remain_stat_points: u16,
    pub base_hp: u32,
    pub base_sp: u32,
}

impl Default for CharacterStatsState {
    fn default() -> Self {
        Self {
            level: 1,
            vit: 3,
            int_: 3,
            str_: 3,
            dex: 3,
            remain_stat_points: 0,
            base_hp: 700,
            base_sp: 250,
        }
    }
}

impl CharacterStatsState {
    pub fn calculate_max_hp(&self) -> u32 {
        self.base_hp + (self.vit as u32 * 40) + (self.level * 20)
    }

    pub fn calculate_max_sp(&self) -> u32 {
        self.base_sp + (self.int_ as u32 * 20) + (self.level * 10)
    }

    pub fn calculate_defense(&self) -> u32 {
        (self.vit as u32) + (self.level * 2)
    }

    pub fn calculate_attack_power(&self) -> u32 {
        (self.str_ as u32 * 2) + (self.level * 2)
    }

    pub fn calculate_evasion_rate(&self) -> u32 {
        ((self.dex as u32) / 3).min(45)
    }

    pub fn can_increase(&self, stat: StatType) -> bool {
        if self.remain_stat_points == 0 {
            return false;
        }
        let cur = match stat {
            StatType::Vit => self.vit,
            StatType::Int => self.int_,
            StatType::Str => self.str_,
            StatType::Dex => self.dex,
        };
        cur < STAT_MAX_LIMIT
    }

    pub fn increase_stat(&mut self, stat: StatType) -> bool {
        if !self.can_increase(stat) {
            return false;
        }
        match stat {
            StatType::Vit => self.vit += 1,
            StatType::Int => self.int_ += 1,
            StatType::Str => self.str_ += 1,
            StatType::Dex => self.dex += 1,
        }
        self.remain_stat_points -= 1;
        true
    }
}

static STATS_STATE: OnceLock<Mutex<CharacterStatsState>> = OnceLock::new();

pub fn get_character_stats_state() -> &'static Mutex<CharacterStatsState> {
    STATS_STATE.get_or_init(|| Mutex::new(CharacterStatsState::default()))
}

// C-FFI exports
#[no_mangle]
pub extern "C" fn m2_game_stats_set_attributes(
    level: u32,
    vit: u16,
    int_: u16,
    str_: u16,
    dex: u16,
    remain_pts: u16,
) {
    safe_ffi_call((), || {
        let mut state = get_character_stats_state().lock().unwrap();
        state.level = level.max(1);
        state.vit = vit;
        state.int_ = int_;
        state.str_ = str_;
        state.dex = dex;
        state.remain_stat_points = remain_pts;
    })
}

#[no_mangle]
pub extern "C" fn m2_game_stats_get_max_hp() -> u32 {
    safe_ffi_call(0, || {
        let state = get_character_stats_state().lock().unwrap();
        state.calculate_max_hp()
    })
}

#[no_mangle]
pub extern "C" fn m2_game_stats_get_max_sp() -> u32 {
    safe_ffi_call(0, || {
        let state = get_character_stats_state().lock().unwrap();
        state.calculate_max_sp()
    })
}

#[no_mangle]
pub extern "C" fn m2_game_stats_get_defense() -> u32 {
    safe_ffi_call(0, || {
        let state = get_character_stats_state().lock().unwrap();
        state.calculate_defense()
    })
}

#[no_mangle]
pub extern "C" fn m2_game_stats_get_attack() -> u32 {
    safe_ffi_call(0, || {
        let state = get_character_stats_state().lock().unwrap();
        state.calculate_attack_power()
    })
}

#[no_mangle]
pub extern "C" fn m2_game_stats_increase(stat_type: u8) -> bool {
    safe_ffi_call(false, || {
        let mut state = get_character_stats_state().lock().unwrap();
        let stat = match stat_type {
            0 => StatType::Vit,
            1 => StatType::Int,
            2 => StatType::Str,
            _ => StatType::Dex,
        };
        state.increase_stat(stat)
    })
}
