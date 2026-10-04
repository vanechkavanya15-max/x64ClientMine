#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PetState {
    Unsummoned,
    Summoned,
    Hungry,
    Evolving,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PetType {
    Type1 = 1,
    Type2 = 2,
    Type3 = 3,
    Type4 = 4,
    Type5 = 5,
    Type6 = 6,
    Type7 = 7,
    Type8 = 8,
}

impl PetType {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            1 => Some(PetType::Type1),
            2 => Some(PetType::Type2),
            3 => Some(PetType::Type3),
            4 => Some(PetType::Type4),
            5 => Some(PetType::Type5),
            6 => Some(PetType::Type6),
            7 => Some(PetType::Type7),
            8 => Some(PetType::Type8),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PetSkill {
    None,
    WarriorResistance,
    Pierce,
    Berserk,
}

#[derive(Debug, Clone)]
pub struct PetStats {
    pub level: u8,
    pub age_days: u32,
    pub exp: u32,
    pub max_exp: u32,
    pub duration_seconds: u32,
    pub max_duration_seconds: u32,
    pub pet_type: PetType,
}

#[derive(Debug, Clone)]
pub struct PetSkills {
    pub slots: [PetSkill; 3],
}

#[derive(Debug, Clone)]
pub struct PetCompanion {
    pub state: PetState,
    pub stats: PetStats,
    pub skills: PetSkills,
    pub id: u32,
}

impl PetCompanion {
    pub fn new(id: u32, pet_type: PetType, max_duration: u32) -> Self {
        Self {
            state: PetState::Unsummoned,
            stats: PetStats {
                level: 1,
                age_days: 0,
                exp: 0,
                max_exp: 100, // Starting max exp
                duration_seconds: max_duration,
                max_duration_seconds: max_duration,
                pet_type,
            },
            skills: PetSkills {
                slots: [PetSkill::None; 3],
            },
            id,
        }
    }

    pub fn handle_summon_packet(&mut self, is_summon: bool) -> bool {
        if is_summon {
            self.summon()
        } else {
            self.unsummon()
        }
    }

    pub fn summon(&mut self) -> bool {
        if self.state == PetState::Unsummoned && self.stats.duration_seconds > 0 {
            if self.stats.duration_seconds < self.stats.max_duration_seconds / 10 {
                self.state = PetState::Hungry;
            } else {
                self.state = PetState::Summoned;
            }
            true
        } else {
            false
        }
    }

    pub fn unsummon(&mut self) -> bool {
        if self.state != PetState::Unsummoned {
            self.state = PetState::Unsummoned;
            true
        } else {
            false
        }
    }

    pub fn tick_duration(&mut self, delta_seconds: u32) {
        if self.state == PetState::Summoned || self.state == PetState::Hungry || self.state == PetState::Evolving {
            self.stats.duration_seconds = self.stats.duration_seconds.saturating_sub(delta_seconds);
            if self.stats.duration_seconds == 0 {
                self.state = PetState::Unsummoned;
            } else if self.stats.duration_seconds < self.stats.max_duration_seconds / 10 {
                if self.state == PetState::Summoned {
                    self.state = PetState::Hungry;
                }
            }
        }
    }

    pub fn feed(&mut self, restore_amount: u32) {
        self.stats.duration_seconds = (self.stats.duration_seconds + restore_amount).min(self.stats.max_duration_seconds);
        if self.stats.duration_seconds >= self.stats.max_duration_seconds / 10 && self.state == PetState::Hungry {
            self.state = PetState::Summoned;
        }
    }

    pub fn add_exp(&mut self, amount: u32) {
        if self.stats.level >= 105 {
            return;
        }
        
        self.stats.exp += amount;
        
        while self.stats.exp >= self.stats.max_exp && self.stats.level < 105 {
            self.stats.exp -= self.stats.max_exp;
            self.stats.level += 1;
            self.stats.max_exp = self.calculate_next_max_exp();
            
            if self.stats.level == 40 || self.stats.level == 80 {
                self.state = PetState::Evolving;
            }
        }
        
        if self.stats.level == 105 {
            self.stats.exp = 0;
        }
    }

    fn calculate_next_max_exp(&self) -> u32 {
        100 + (self.stats.level as u32 * 50)
    }

    pub fn complete_evolution(&mut self) {
        if self.state == PetState::Evolving {
            if self.stats.duration_seconds < self.stats.max_duration_seconds / 10 {
                self.state = PetState::Hungry;
            } else {
                self.state = PetState::Summoned;
            }
        }
    }

    pub fn set_skill(&mut self, slot: usize, skill: PetSkill) -> bool {
        if slot < 3 {
            self.skills.slots[slot] = skill;
            true
        } else {
            false
        }
    }

    pub fn get_skill(&self, slot: usize) -> Option<PetSkill> {
        if slot < 3 {
            Some(self.skills.slots[slot])
        } else {
            None
        }
    }

    pub fn increment_age(&mut self) {
        self.stats.age_days += 1;
    }
}
