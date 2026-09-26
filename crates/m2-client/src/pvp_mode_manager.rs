#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PvpMode {
    Peace,
    Revenge,
    Guild,
    Free,
    Protect,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlignmentType {
    Normal,
    White,
    Dark,
}

pub struct PvpModeManager;

impl PvpModeManager {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_alignment_grade(alignment: i32) -> u32 {
        if alignment >= 12000 {
            0
        } else if alignment >= 8000 {
            1
        } else if alignment >= 4000 {
            2
        } else if alignment >= 1000 {
            3
        } else if alignment >= 0 {
            4
        } else if alignment > -4000 {
            5
        } else if alignment > -8000 {
            6
        } else if alignment > -12000 {
            7
        } else {
            8
        }
    }

    pub fn get_alignment_type(alignment: i32) -> AlignmentType {
        let grade = Self::get_alignment_grade(alignment);
        match grade {
            0..=3 => AlignmentType::White,
            5..=8 => AlignmentType::Dark,
            _ => AlignmentType::Normal,
        }
    }

    pub fn is_conflict_alignment(my_alignment: i32, target_alignment: i32, target_pvp_mode: PvpMode) -> bool {
        if target_pvp_mode == PvpMode::Protect {
            return false;
        }

        let my_type = Self::get_alignment_type(my_alignment);
        let target_type = Self::get_alignment_type(target_alignment);

        match my_type {
            AlignmentType::Normal | AlignmentType::White => target_type == AlignmentType::Dark,
            AlignmentType::Dark => my_type != target_type,
        }
    }

    pub fn can_attack_player(
        my_empire: u8,
        my_guild: u32,
        my_pvp_mode: PvpMode,
        my_alignment: i32,
        target_empire: u8,
        target_guild: u32,
        target_pvp_mode: PvpMode,
        target_alignment: i32,
        is_same_party: bool,
        is_killer: bool,
        is_duel_active: bool,
        is_pvp_instance: bool,
    ) -> bool {
        if is_duel_active {
            return true;
        }

        if my_pvp_mode == PvpMode::Guild {
            if my_guild != 0 && my_guild == target_guild {
                return false;
            }
        }

        if is_killer {
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
                        if my_guild == 0 || my_guild != target_guild {
                            return true;
                        }
                    }
                }
            }
        }

        if my_empire == target_empire {
            if is_pvp_instance {
                return true;
            }

            if my_pvp_mode == PvpMode::Revenge {
                if !is_same_party {
                    if (my_guild == 0 || my_guild != target_guild)
                        && Self::is_conflict_alignment(my_alignment, target_alignment, target_pvp_mode)
                        && target_alignment < 0
                    {
                        return true;
                    }
                }
            }
        } else {
            return true;
        }

        false
    }

    pub fn calculate_alignment_penalty(
        my_pvp_mode: PvpMode,
        _my_level: u32,
        target_alignment: i32,
    ) -> i32 {
        if my_pvp_mode == PvpMode::Free && target_alignment >= 0 {
            // Formula in general: alignment penalty = level * -something
            // For now, returning a standard generic penalty value: -2000
            // Because Metin2 alignment penalty calculation logic is usually server-side,
            // but the prompt explicitly requires it. The standard Metin2 penalty formula is often simply fixed or level dependent.
            // Let's implement a formula: -2000 penalty for innocent kill
            return -2000;
        }
        0
    }
}
