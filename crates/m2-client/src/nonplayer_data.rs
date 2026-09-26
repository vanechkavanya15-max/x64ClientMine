use std::collections::HashMap;

// Rangi potworow
pub const RANK_PAWN: u8 = 0;
pub const RANK_KNIGHT: u8 = 1;
pub const RANK_S_KNIGHT: u8 = 2;
pub const RANK_BOSS: u8 = 3;
pub const RANK_KING: u8 = 4;

// Typy potworow
pub const TYPE_MONSTER: u8 = 0;
pub const TYPE_NPC: u8 = 1;
pub const TYPE_WARP: u8 = 2;
pub const TYPE_PET: u8 = 3;

#[derive(Debug, Clone)]
pub struct NonPlayerData {
    pub race_vnum: u32,
    pub mob_name: String,
    pub rank: u8,
    pub mob_type: u8,
    pub is_aggressive: bool,
    pub attack_speed: u32,
    pub move_speed: u32,
}

pub struct NonPlayerDatabase {
    pub mob_table: HashMap<u32, NonPlayerData>,
}

impl NonPlayerDatabase {
    pub fn new() -> Self {
        Self {
            mob_table: HashMap::new(),
        }
    }

    pub fn register_mob(&mut self, mob: NonPlayerData) {
        self.mob_table.insert(mob.race_vnum, mob);
    }

    pub fn get_monster_name(&self, vnum: u32) -> &str {
        if let Some(mob) = self.mob_table.get(&vnum) {
            &mob.mob_name
        } else {
            ""
        }
    }

    pub fn is_boss(&self, vnum: u32) -> bool {
        if let Some(mob) = self.mob_table.get(&vnum) {
            mob.rank == RANK_BOSS || mob.rank == RANK_KING
        } else {
            false
        }
    }

    pub fn is_aggressive(&self, vnum: u32) -> bool {
        if let Some(mob) = self.mob_table.get(&vnum) {
            mob.is_aggressive
        } else {
            false
        }
    }
}
