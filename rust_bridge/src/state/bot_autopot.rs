use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

pub const BOT_AUTOPOT: u8 = 0;
pub const POTION_COOLDOWN_MS: u32 = 1000;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct AutoPotEngine {
    pub hp_threshold_pct: u8,
    pub mp_threshold_pct: u8,
    pub is_enabled: bool,
    last_pot_time: u32,
}

impl AutoPotEngine {
    pub fn new() -> Self {
        Self {
            hp_threshold_pct: 0,
            mp_threshold_pct: 0,
            is_enabled: false,
            last_pot_time: 0,
        }
    }

    pub fn should_pot_hp(&self, current_hp_pct: u8, current_time: u32) -> bool {
        if !self.is_enabled {
            return false;
        }

        if current_hp_pct > self.hp_threshold_pct {
            return false;
        }

        if current_time >= self.last_pot_time {
            // Unikniecie overflow
            if current_time - self.last_pot_time < POTION_COOLDOWN_MS {
                return false;
            }
        } else {
            // Wrap-around
            // elapsed = (u32::MAX - last_pot_time) + 1 + current_time
            let elapsed = u32::MAX.saturating_sub(self.last_pot_time).saturating_add(1).saturating_add(current_time);
            if elapsed < POTION_COOLDOWN_MS {
                return false;
            }
        }

        true
    }

    pub fn register_pot_used(&mut self, current_time: u32) {
        self.last_pot_time = current_time;
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PotionConfig {
    pub enabled: bool,
    pub threshold_percent: u8,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AutopotConfig {
    pub hp: PotionConfig,
    pub mp: PotionConfig,
}

pub struct AutopotManager {
    configs: HashMap<u32, AutopotConfig>,
}

impl AutopotManager {
    pub fn new() -> Self {
        Self {
            configs: HashMap::new(),
        }
    }

    pub fn set_hp_config(&mut self, vid: u32, enabled: bool, mut threshold_percent: u8) {
        if threshold_percent > 100 {
            threshold_percent = 100;
        }
        let config = self.configs.entry(vid).or_default();
        config.hp.enabled = enabled;
        config.hp.threshold_percent = threshold_percent;
    }

    pub fn set_mp_config(&mut self, vid: u32, enabled: bool, mut threshold_percent: u8) {
        if threshold_percent > 100 {
            threshold_percent = 100;
        }
        let config = self.configs.entry(vid).or_default();
        config.mp.enabled = enabled;
        config.mp.threshold_percent = threshold_percent;
    }

    pub fn get_config(&self, vid: u32) -> Option<&AutopotConfig> {
        self.configs.get(&vid)
    }

    pub fn remove_config(&mut self, vid: u32) {
        self.configs.remove(&vid);
    }

    pub fn should_use_hp(&self, vid: u32, current_hp: u32, max_hp: u32) -> bool {
        if max_hp == 0 {
            return false;
        }

        if let Some(config) = self.configs.get(&vid) {
            if !config.hp.enabled {
                return false;
            }

            let mut current_percent = (current_hp as u64 * 100) / (max_hp as u64);
            if current_percent > 100 {
                current_percent = 100;
            }
            return (current_percent as u8) <= config.hp.threshold_percent;
        }

        false
    }

    pub fn should_use_mp(&self, vid: u32, current_mp: u32, max_mp: u32) -> bool {
        if max_mp == 0 {
            return false;
        }

        if let Some(config) = self.configs.get(&vid) {
            if !config.mp.enabled {
                return false;
            }

            let mut current_percent = (current_mp as u64 * 100) / (max_mp as u64);
            if current_percent > 100 {
                current_percent = 100;
            }
            return (current_percent as u8) <= config.mp.threshold_percent;
        }

        false
    }
}

use once_cell::sync::Lazy;
use std::sync::Mutex;

static GLOBAL_ENGINE: Lazy<Mutex<AutoPotEngine>> = Lazy::new(|| Mutex::new(AutoPotEngine::new()));

#[no_mangle]
pub unsafe extern "C" fn m2_autopot_create() -> *mut AutoPotEngine {
    std::panic::catch_unwind(|| {
        let engine = Box::new(AutoPotEngine::new());
        Box::into_raw(engine)
    }).unwrap_or(std::ptr::null_mut())
}

#[no_mangle]
pub unsafe extern "C" fn m2_autopot_free(ptr: *mut AutoPotEngine) {
    let _ = std::panic::catch_unwind(|| {
        if !ptr.is_null() {
            let _ = Box::from_raw(ptr);
        }
    });
}

#[no_mangle]
pub unsafe extern "C" fn m2_autopot_check(hp_pct: u8) -> bool {
    let result = std::panic::catch_unwind(|| {
        let mut engine = GLOBAL_ENGINE.lock().unwrap();
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u32;
        
        let should_pot = engine.should_pot_hp(hp_pct, now);
        if should_pot {
            engine.register_pot_used(now);
        }
        should_pot
    });
    result.unwrap_or(false)
}

#[no_mangle]
pub unsafe extern "C" fn m2_autopot_set_config(hp_threshold: u8, mp_threshold: u8, enabled: bool) {
    let _ = std::panic::catch_unwind(|| {
        let mut engine = GLOBAL_ENGINE.lock().unwrap();
        engine.hp_threshold_pct = hp_threshold;
        engine.mp_threshold_pct = mp_threshold;
        engine.is_enabled = enabled;
    });
}
