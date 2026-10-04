use std::sync::Mutex;
use std::time::{Instant, Duration};
use once_cell::sync::Lazy;

#[derive(Debug, Clone, Copy)]
pub struct AutoPotionInfo {
    pub is_activated: bool,
    pub current_amount: i32,
    pub total_amount: i32,
    pub inventory_slot_index: i32,
}

impl Default for AutoPotionInfo {
    fn default() -> Self {
        Self {
            is_activated: false,
            current_amount: 0,
            total_amount: 0,
            inventory_slot_index: 0,
        }
    }
}

pub struct GameAutoPotionManager {
    pub hp_info: AutoPotionInfo,
    pub sp_info: AutoPotionInfo,
    pub last_hp_use_time: Option<Instant>,
    pub last_sp_use_time: Option<Instant>,
    pub hp_threshold_pct: f32,
    pub sp_threshold_pct: f32,
    pub cooldown: Duration,
}

impl GameAutoPotionManager {
    pub fn new() -> Self {
        Self {
            hp_info: AutoPotionInfo::default(),
            sp_info: AutoPotionInfo::default(),
            last_hp_use_time: None,
            last_sp_use_time: None,
            hp_threshold_pct: 0.5,
            sp_threshold_pct: 0.5,
            cooldown: Duration::from_millis(250),
        }
    }

    pub fn set_info(&mut self, potion_type: u32, is_activated: bool, current_amount: i32, total_amount: i32, inventory_slot_index: i32) {
        let info = if potion_type == 0 {
            &mut self.hp_info
        } else {
            &mut self.sp_info
        };
        info.is_activated = is_activated;
        info.current_amount = current_amount;
        info.total_amount = total_amount;
        info.inventory_slot_index = inventory_slot_index;
    }

    pub fn get_info(&self, potion_type: u32) -> Option<AutoPotionInfo> {
        if potion_type == 0 {
            Some(self.hp_info)
        } else if potion_type == 1 {
            Some(self.sp_info)
        } else {
            None
        }
    }

    pub fn update(&mut self, hp_pct: f32, sp_pct: f32) -> (Option<u16>, Option<u16>) {
        let now = Instant::now();
        let mut hp_trigger_slot = None;
        let mut sp_trigger_slot = None;

        if self.hp_info.is_activated && hp_pct < self.hp_threshold_pct {
            if let Some(last_use) = self.last_hp_use_time {
                if now.duration_since(last_use) >= self.cooldown {
                    hp_trigger_slot = Some(self.hp_info.inventory_slot_index as u16);
                    self.last_hp_use_time = Some(now);
                }
            } else {
                hp_trigger_slot = Some(self.hp_info.inventory_slot_index as u16);
                self.last_hp_use_time = Some(now);
            }
        }

        if self.sp_info.is_activated && sp_pct < self.sp_threshold_pct {
            if let Some(last_use) = self.last_sp_use_time {
                if now.duration_since(last_use) >= self.cooldown {
                    sp_trigger_slot = Some(self.sp_info.inventory_slot_index as u16);
                    self.last_sp_use_time = Some(now);
                }
            } else {
                sp_trigger_slot = Some(self.sp_info.inventory_slot_index as u16);
                self.last_sp_use_time = Some(now);
            }
        }

        (hp_trigger_slot, sp_trigger_slot)
    }
}

impl Default for GameAutoPotionManager {
    fn default() -> Self {
        Self::new()
    }
}

pub static AUTO_POTION_MANAGER: Lazy<Mutex<GameAutoPotionManager>> = Lazy::new(|| Mutex::new(GameAutoPotionManager::new()));

#[no_mangle]
pub extern "C" fn m2_py_player_set_auto_potion_info(potion_type: u32, is_activated: bool, current_amount: i32, total_amount: i32, inventory_slot_index: i32) -> bool {
    if potion_type > 1 {
        return false;
    }
    
    if let Ok(mut manager) = AUTO_POTION_MANAGER.lock() {
        manager.set_info(potion_type, is_activated, current_amount, total_amount, inventory_slot_index);
        true
    } else {
        false
    }
}

#[no_mangle]
pub extern "C" fn m2_py_player_get_auto_potion_info(
    potion_type: u32, 
    out_is_activated: *mut bool, 
    out_current_amount: *mut i32, 
    out_total_amount: *mut i32, 
    out_inventory_slot_index: *mut i32
) -> bool {
    if potion_type > 1 {
        return false;
    }

    if out_is_activated.is_null() || out_current_amount.is_null() || out_total_amount.is_null() || out_inventory_slot_index.is_null() {
        return false;
    }

    if let Ok(manager) = AUTO_POTION_MANAGER.lock() {
        if let Some(info) = manager.get_info(potion_type) {
            unsafe {
                *out_is_activated = info.is_activated;
                *out_current_amount = info.current_amount;
                *out_total_amount = info.total_amount;
                *out_inventory_slot_index = info.inventory_slot_index;
            }
            return true;
        }
    }
    false
}
