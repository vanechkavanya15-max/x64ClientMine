// mining_vein_drill_state_v2.rs
// Implementation of Metin2 Mining/Vein Drill System State 1:1 mirroring C++ code

use std::sync::{Mutex, OnceLock};
use rand::Rng;

pub const SKILL_INDEX_MINING: u32 = 124;

/// VNUMs for Pickaxes (0-9 upgrades)
pub const PICKAXE_VNUM_MIN: u32 = 29101;
pub const PICKAXE_VNUM_MAX: u32 = 29110;

/// VNUMs for Veins (Resources)
pub const VEIN_VNUM_MIN: u32 = 20047;
pub const VEIN_VNUM_MAX: u32 = 20072;

/// Digging standard duration
pub const MINING_DEFAULT_DURATION_MS: u64 = 10000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MiningDrillState {
    Idle,
    Digging {
        target_vid: u32,
        vein_vnum: u32,
        start_time_ms: u64,
        duration_ms: u64,
    },
    Success,
    Failed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MiningDrillResult {
    pub success: bool,
    pub target_vid: u32,
    pub vein_vnum: u32,
}

pub struct MiningVeinDrillManager {
    pub state: MiningDrillState,
    pub pickaxe_points: u32,
    pub pickaxe_max_points: u32,
}

impl Default for MiningVeinDrillManager {
    fn default() -> Self {
        Self::new()
    }
}

impl MiningVeinDrillManager {
    pub fn new() -> Self {
        Self {
            state: MiningDrillState::Idle,
            pickaxe_points: 0,
            pickaxe_max_points: 6000,
        }
    }

    pub fn is_pickaxe(vnum: u32) -> bool {
        vnum >= PICKAXE_VNUM_MIN && vnum <= PICKAXE_VNUM_MAX
    }

    pub fn is_vein(vnum: u32) -> bool {
        vnum >= VEIN_VNUM_MIN && vnum <= VEIN_VNUM_MAX
    }

    pub fn can_dig(
        &self,
        is_mounted: bool,
        is_dead: bool,
        is_stunned: bool,
        weapon_vnum: u32,
        target_vnum: u32,
    ) -> Result<(), &'static str> {
        if is_dead || is_stunned {
            return Err("Cannot mine while dead or stunned");
        }

        if is_mounted {
            return Err("Cannot mine while mounted");
        }

        if !Self::is_pickaxe(weapon_vnum) {
            return Err("Not holding a pickaxe");
        }

        if !Self::is_vein(target_vnum) {
            return Err("Target is not a resource vein");
        }

        if self.state != MiningDrillState::Idle {
            return Err("Already digging");
        }

        Ok(())
    }

    pub fn start_dig(&mut self, target_vid: u32, vein_vnum: u32, now_ms: u64) -> bool {
        if self.state != MiningDrillState::Idle {
            return false;
        }

        self.state = MiningDrillState::Digging {
            target_vid,
            vein_vnum,
            start_time_ms: now_ms,
            duration_ms: MINING_DEFAULT_DURATION_MS,
        };
        true
    }

    pub fn cancel_dig(&mut self) {
        self.state = MiningDrillState::Idle;
    }

    pub fn update(&mut self, now_ms: u64, skill_level: u32) -> Option<MiningDrillResult> {
        if let MiningDrillState::Digging { target_vid, vein_vnum, start_time_ms, duration_ms } = self.state {
            if now_ms >= start_time_ms + duration_ms {
                let success = self.calculate_success(vein_vnum, skill_level);
                
                if success {
                    self.state = MiningDrillState::Success;
                    if self.pickaxe_points < self.pickaxe_max_points {
                        self.pickaxe_points += 1;
                    }
                } else {
                    self.state = MiningDrillState::Failed;
                }

                return Some(MiningDrillResult {
                    success,
                    target_vid,
                    vein_vnum,
                });
            }
        }
        None
    }

    fn calculate_success(&self, _vein_vnum: u32, skill_level: u32) -> bool {
        // Base chance 150/1000 = 15%
        // + 0.5% per skill level
        let base_chance: u32 = 150;
        let skill_bonus = skill_level * 5;
        let total_chance = (base_chance + skill_bonus).min(900); // max 90%
        
        // Accurate true probability roll using rand crate replacing the deterministic stub
        let mut rng = rand::thread_rng();
        let roll = rng.gen_range(0..1000);
        
        roll < total_chance
    }
}

static MANAGER: OnceLock<Mutex<MiningVeinDrillManager>> = OnceLock::new();

pub fn get_manager() -> &'static Mutex<MiningVeinDrillManager> {
    MANAGER.get_or_init(|| Mutex::new(MiningVeinDrillManager::new()))
}

// C-FFI
#[no_mangle]
pub unsafe extern "C" fn m2_mining_drill_is_pickaxe(vnum: u32) -> bool {
    MiningVeinDrillManager::is_pickaxe(vnum)
}

#[no_mangle]
pub unsafe extern "C" fn m2_mining_drill_is_vein(vnum: u32) -> bool {
    MiningVeinDrillManager::is_vein(vnum)
}

#[no_mangle]
pub unsafe extern "C" fn m2_mining_drill_start(target_vid: u32, vein_vnum: u32, now_ms: u64) -> bool {
    if let Ok(mut mgr) = get_manager().lock() {
        mgr.start_dig(target_vid, vein_vnum, now_ms)
    } else {
        false
    }
}

#[no_mangle]
pub unsafe extern "C" fn m2_mining_drill_cancel() {
    if let Ok(mut mgr) = get_manager().lock() {
        mgr.cancel_dig();
    }
}

#[no_mangle]
pub unsafe extern "C" fn m2_mining_drill_update(now_ms: u64, skill_level: u32) -> bool {
    if let Ok(mut mgr) = get_manager().lock() {
        if let Some(res) = mgr.update(now_ms, skill_level) {
            return res.success;
        }
    }
    false
}
