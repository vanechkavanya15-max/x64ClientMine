#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TargetType {
    Player,
    Monster,
    Metin,
    Npc,
    TrainingDummy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PvpMode {
    Peace,
    Revenge,
    Free,
    Guild,
}

#[derive(Debug, Clone)]
pub struct InstanceFlags {
    pub is_dead: bool,
    pub is_stun: bool,
    pub is_sleep: bool,
    pub is_paralyzed: bool,
    pub is_fishing: bool,
    pub is_mining: bool,
    pub is_shop_open: bool,
}

impl Default for InstanceFlags {
    fn default() -> Self {
        Self {
            is_dead: false,
            is_stun: false,
            is_sleep: false,
            is_paralyzed: false,
            is_fishing: false,
            is_mining: false,
            is_shop_open: false,
        }
    }
}

pub struct BattleInstance {
    pub flags: InstanceFlags,
    pub is_in_safe_zone: bool,
    pub pvp_mode: PvpMode,
    pub kingdom_id: u8,
    pub target_type: TargetType,
}

impl BattleInstance {
    pub fn new(target_type: TargetType, kingdom_id: u8) -> Self {
        Self {
            flags: InstanceFlags::default(),
            is_in_safe_zone: false,
            pvp_mode: PvpMode::Peace,
            kingdom_id,
            target_type,
        }
    }

    /// Checks if the instance is capable of performing any action.
    pub fn can_act(&self) -> bool {
        !self.flags.is_dead
            && !self.flags.is_stun
            && !self.flags.is_sleep
            && !self.flags.is_paralyzed
            && !self.flags.is_fishing
            && !self.flags.is_mining
            && !self.flags.is_shop_open
    }

    /// Checks if the instance can attack a specific target.
    pub fn can_attack(&self, target: &BattleInstance) -> bool {
        // 1. Cannot attack if incapable of acting
        if !self.can_act() {
            return false;
        }

        // Target cannot be attacked if it is dead
        if target.flags.is_dead {
            return false;
        }

        // 2. Safe zone verification
        // In a safe zone, only training dummies can be attacked.
        if self.is_in_safe_zone {
            if target.target_type != TargetType::TrainingDummy {
                return false;
            }
        }
        
        // Target is in a safe zone (and not a training dummy)
        if target.is_in_safe_zone {
            if target.target_type != TargetType::TrainingDummy {
                return false;
            }
        }

        // NPCs generally cannot be attacked
        if target.target_type == TargetType::Npc {
            return false;
        }

        // 3. PvP Rules
        if self.target_type == TargetType::Player && target.target_type == TargetType::Player {
            match self.pvp_mode {
                PvpMode::Peace => {
                    // Cannot attack players from the same kingdom in PEACE mode
                    if self.kingdom_id == target.kingdom_id {
                        return false;
                    }
                }
                _ => {} // Other modes allow attacks based on their own rules (simplified here for Revenge, Free, Guild to allow)
            }
        }

        true
    }
}
