use std::collections::HashSet;
use std::sync::{OnceLock, RwLock};

#[derive(Debug, Default)]
pub struct PvpGvgManagerV2 {
    pvp_keys: HashSet<(u32, u32)>,
    pvp_ready_keys: HashSet<(u32, u32)>,
    gvg_keys: HashSet<(u32, u32)>,
    duel_keys: HashSet<(u32, u32)>,
}

impl PvpGvgManagerV2 {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn clear_pvp_key_system(&mut self) {
        self.pvp_ready_keys.clear();
        self.pvp_keys.clear();
        self.gvg_keys.clear();
        self.duel_keys.clear();
    }

    fn get_key(id1: u32, id2: u32) -> (u32, u32) {
        (std::cmp::min(id1, id2), std::cmp::max(id1, id2))
    }

    pub fn insert_pvp_key(&mut self, src_vid: u32, dst_vid: u32) {
        let key = Self::get_key(src_vid, dst_vid);
        self.pvp_keys.insert(key);
    }

    pub fn insert_pvp_ready_key(&mut self, src_vid: u32, dst_vid: u32) {
        let key = Self::get_key(src_vid, dst_vid);
        self.pvp_keys.insert(key); // As in C++: g_kSet_dwPVPKey.insert(dwPVPReadyKey);
    }

    pub fn remove_pvp_key(&mut self, src_vid: u32, dst_vid: u32) {
        let key = Self::get_key(src_vid, dst_vid);
        self.pvp_keys.remove(&key);
    }

    pub fn insert_gvg_key(&mut self, src_guild: u32, dst_guild: u32) {
        let key = Self::get_key(src_guild, dst_guild);
        self.gvg_keys.insert(key);
    }

    pub fn remove_gvg_key(&mut self, src_guild: u32, dst_guild: u32) {
        let key = Self::get_key(src_guild, dst_guild);
        self.gvg_keys.remove(&key);
    }

    pub fn insert_duel_key(&mut self, src_vid: u32, dst_vid: u32) {
        let key = Self::get_key(src_vid, dst_vid);
        self.duel_keys.insert(key);
    }

    pub fn has_pvp_key(&self, src_vid: u32, dst_vid: u32) -> bool {
        let key = Self::get_key(src_vid, dst_vid);
        self.pvp_keys.contains(&key)
    }

    pub fn has_pvp_ready_key(&self, src_vid: u32, dst_vid: u32) -> bool {
        let key = Self::get_key(src_vid, dst_vid);
        // Note: C++ has `__FindPVPReadyKey` which checks `g_kSet_dwPVPReadyKey`.
        // However `InsertPVPReadyKey` inserts into `g_kSet_dwPVPKey`.
        // To be strictly correct to C++ logic in `__FindPVPReadyKey`, we should check `self.pvp_ready_keys`.
        self.pvp_ready_keys.contains(&key)
    }

    pub fn has_gvg_key(&self, src_guild: u32, dst_guild: u32) -> bool {
        let key = Self::get_key(src_guild, dst_guild);
        self.gvg_keys.contains(&key)
    }

    pub fn has_duel_key(&self, src_vid: u32, dst_vid: u32) -> bool {
        let key = Self::get_key(src_vid, dst_vid);
        self.duel_keys.contains(&key)
    }
}

static PVP_GVG_MANAGER: OnceLock<RwLock<PvpGvgManagerV2>> = OnceLock::new();

pub fn get_pvp_gvg_manager() -> &'static RwLock<PvpGvgManagerV2> {
    PVP_GVG_MANAGER.get_or_init(|| RwLock::new(PvpGvgManagerV2::new()))
}

use crate::pvp_mode_manager::{AlignmentType, PvpMode, PvpModeManager};
use m2_client_bridge::ffi_guard::safe_ffi_call;
use crate::character_manager_core::get_character_manager;

// Constants mapping to C++ DUEL modes
pub const DUEL_NONE: u32 = 0;
pub const DUEL_CANNOTATTACK: u32 = 1;
pub const DUEL_START: u32 = 2;

// Additional trait/methods would normally be on InstanceBase, but we can pass properties directly to the FFI function.
#[no_mangle]
pub extern "C" fn m2_pvp_can_attack(
    src_vid: u32,
    dst_vid: u32,
    
    // Attacker properties
    src_is_main: bool,
    src_is_observer: bool,
    src_is_stone: bool,
    src_is_pc: bool,
    src_is_enemy: bool,
    src_is_poly: bool,
    src_duel_mode: u32,
    src_pk_mode: u8,
    src_guild_id: u32,
    src_empire_id: u32,
    src_alignment: i32,

    // Victim properties
    dst_is_pc: bool,
    dst_is_stone: bool,
    dst_is_wooden_door: bool,
    dst_is_enemy: bool,
    dst_is_building: bool,
    dst_pk_mode: u8,
    dst_guild_id: u32,
    dst_empire_id: u32,
    dst_alignment: i32,
    dst_is_killer: bool,
    
    // Shared state properties
    is_same_party: bool,
) -> bool {
    safe_ffi_call(false, || {
        if src_is_main && src_is_observer {
            return false;
        }

        if src_vid == dst_vid {
            return false;
        }

        let mgr_lock = get_pvp_gvg_manager().read().unwrap();

        if src_is_stone {
            if dst_is_pc {
                return true;
            }
        } else if src_is_pc {
            if dst_is_stone {
                return true;
            }

            if dst_is_pc {
                if src_duel_mode != DUEL_NONE {
                    match src_duel_mode {
                        DUEL_CANNOTATTACK => return false,
                        DUEL_START => {
                            if mgr_lock.has_duel_key(src_vid, dst_vid) {
                                return true;
                            } else {
                                return false;
                            }
                        }
                        _ => {}
                    }
                }

                let my_pvp_mode = match src_pk_mode {
                    0 => PvpMode::Peace,
                    1 => PvpMode::Revenge,
                    2 => PvpMode::Free,
                    3 => PvpMode::Protect,
                    4 => PvpMode::Guild,
                    _ => PvpMode::Peace,
                };
                
                let target_pvp_mode = match dst_pk_mode {
                    0 => PvpMode::Peace,
                    1 => PvpMode::Revenge,
                    2 => PvpMode::Free,
                    3 => PvpMode::Protect,
                    4 => PvpMode::Guild,
                    _ => PvpMode::Peace,
                };

                if my_pvp_mode == PvpMode::Guild {
                    if src_guild_id != 0 && src_guild_id == dst_guild_id {
                        return false;
                    }
                }

                if dst_is_killer {
                    if !is_same_party {
                        return true;
                    }
                }

                if my_pvp_mode != PvpMode::Protect {
                    if my_pvp_mode == PvpMode::Free {
                        if target_pvp_mode != PvpMode::Protect {
                            if !is_same_party {
                                return true;
                            }
                        }
                    }
                    if my_pvp_mode == PvpMode::Guild {
                        if target_pvp_mode != PvpMode::Protect {
                            if !is_same_party {
                                if src_guild_id == 0 || src_guild_id != dst_guild_id {
                                    return true;
                                }
                            }
                        }
                    }
                }

                let is_same_empire = (dst_empire_id == 0) || (dst_empire_id == src_empire_id); // Simplified IsSameEmpire (ignoring GM here as we don't have GM flag explicitly, although can assume same logic)
                
                if is_same_empire {
                    // IsPVPInstance check
                    let is_pvp_instance = if src_duel_mode != DUEL_NONE {
                        true
                    } else {
                        mgr_lock.has_pvp_key(src_vid, dst_vid) || (src_guild_id != 0 && dst_guild_id != 0 && mgr_lock.has_gvg_key(src_guild_id, dst_guild_id))
                    };

                    if is_pvp_instance {
                        return true;
                    }

                    if my_pvp_mode == PvpMode::Revenge {
                        if !is_same_party {
                            if (src_guild_id == 0 || src_guild_id != dst_guild_id) &&
                               PvpModeManager::is_conflict_alignment(src_alignment, dst_alignment, target_pvp_mode) &&
                               dst_alignment < 0
                            {
                                return true;
                            }
                        }
                    }
                } else {
                    return true;
                }
            }

            if dst_is_enemy {
                return true;
            }

            if dst_is_wooden_door {
                return true;
            }
        } else if src_is_enemy {
            if dst_is_pc {
                return true;
            }
            if dst_is_building {
                return true;
            }
        } else if src_is_poly {
            if dst_is_pc {
                return true;
            }
            if dst_is_enemy {
                return true;
            }
        }

        false
    })
}
