use std::sync::{Mutex, OnceLock};
use std::panic::catch_unwind;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StoneCategory {
    Diamond,
    Ruby,
    Jade,
    Sapphire,
    Garnet,
    Onyx,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DragonSoulGrade {
    Normal,
    Brilliant,
    Rare,
    Ancient,
    Legendary,
}

impl DragonSoulGrade {
    pub fn next(&self) -> Option<Self> {
        match self {
            DragonSoulGrade::Normal => Some(DragonSoulGrade::Brilliant),
            DragonSoulGrade::Brilliant => Some(DragonSoulGrade::Rare),
            DragonSoulGrade::Rare => Some(DragonSoulGrade::Ancient),
            DragonSoulGrade::Ancient => Some(DragonSoulGrade::Legendary),
            DragonSoulGrade::Legendary => None,
        }
    }
}

/// Stopien czystosci smoczego kamienia (Dragon Soul Clarity / Step)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum DragonSoulClarity {
    Lowest = 0,   // Matowy / Klarowny
    Low = 1,      // Przejrzysty
    Mid = 2,      // Bez skazy
    High = 3,     // Znakomity
    Highest = 4,  // Wyborny / Perfekcyjny
}

impl DragonSoulClarity {
    pub fn next(&self) -> Option<Self> {
        match self {
            DragonSoulClarity::Lowest => Some(DragonSoulClarity::Low),
            DragonSoulClarity::Low => Some(DragonSoulClarity::Mid),
            DragonSoulClarity::Mid => Some(DragonSoulClarity::High),
            DragonSoulClarity::High => Some(DragonSoulClarity::Highest),
            DragonSoulClarity::Highest => None,
        }
    }
}

pub const DS_MAX_REFINE_LEVEL: u8 = 6;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DeckType {
    Deck1,
    Deck2,
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Bonus {
    pub attack: i32,
    pub defense: i32,
}

impl Bonus {
    pub fn add(&self, other: &Bonus) -> Bonus {
        Bonus {
            attack: self.attack + other.attack,
            defense: self.defense + other.defense,
        }
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPlayerItemAttribute {
    pub b_type: u8,
    pub s_value: i16,
}

impl Default for TPlayerItemAttribute {
    fn default() -> Self {
        Self { b_type: 0, s_value: 0 }
    }
}

/// Zgodna 1:1 z C++ TItemData reprezentacja kamienia smoczej duszy z socketami i 7 atrybutami
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DragonSoulItemData {
    pub vnum: u32,
    pub count: u8,
    pub flags: u32,
    pub anti_flags: u32,
    pub sockets: [i32; 3],
    pub attrs: [TPlayerItemAttribute; 7],
}

impl Default for DragonSoulItemData {
    fn default() -> Self {
        Self {
            vnum: 0,
            count: 0,
            flags: 0,
            anti_flags: 0,
            sockets: [0; 3],
            attrs: [TPlayerItemAttribute::default(); 7],
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DragonSoulStone {
    pub category: StoneCategory,
    pub grade: DragonSoulGrade,
    pub bonus: Bonus,
}

/// Rozszerzona reprezentacja kamienia ze stopniem czystosci i poziomem ulepszenia +0..+6
#[derive(Debug, Clone, PartialEq)]
pub struct DragonSoulRefineStone {
    pub category: StoneCategory,
    pub grade: DragonSoulGrade,
    pub clarity: DragonSoulClarity,
    pub level: u8, // 0..=6
    pub bonus: Bonus,
}

impl DragonSoulRefineStone {
    pub fn new(
        category: StoneCategory,
        grade: DragonSoulGrade,
        clarity: DragonSoulClarity,
        level: u8,
        bonus: Bonus,
    ) -> Self {
        Self {
            category,
            grade,
            clarity,
            level: level.min(DS_MAX_REFINE_LEVEL),
            bonus,
        }
    }

    pub fn from_base(stone: &DragonSoulStone) -> Self {
        Self {
            category: stone.category,
            grade: stone.grade,
            clarity: DragonSoulClarity::Lowest,
            level: 0,
            bonus: stone.bonus,
        }
    }

    pub fn to_base(&self) -> DragonSoulStone {
        DragonSoulStone {
            category: self.category,
            grade: self.grade,
            bonus: self.bonus,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DragonSoulRefineType {
    Grade,    // Stopien jakosci
    Clarity,  // Stopien czystosci
    Level,    // Poziom (+0..+6)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DragonSoulRefineError {
    NotEnoughStones,
    MismatchedCategory,
    MismatchedGrade,
    MismatchedClarity,
    MaxGradeReached,
    MaxClarityReached,
    MaxLevelReached,
    InvalidBeanItem,
    RefineFailed,
}

pub struct DragonSoulRefine;

impl DragonSoulRefine {
    /// Generuje kanoniczny VNUM smoczego kamienia
    pub fn make_vnum(
        category: StoneCategory,
        grade: DragonSoulGrade,
        clarity: DragonSoulClarity,
        level: u8,
    ) -> u32 {
        let cat_idx = match category {
            StoneCategory::Diamond => 1,
            StoneCategory::Ruby => 2,
            StoneCategory::Jade => 3,
            StoneCategory::Sapphire => 4,
            StoneCategory::Garnet => 5,
            StoneCategory::Onyx => 6,
        };
        let grade_idx = match grade {
            DragonSoulGrade::Normal => 0,
            DragonSoulGrade::Brilliant => 1,
            DragonSoulGrade::Rare => 2,
            DragonSoulGrade::Ancient => 3,
            DragonSoulGrade::Legendary => 4,
        };
        let clarity_idx = clarity as u32;
        let lvl = (level.min(DS_MAX_REFINE_LEVEL)) as u32;
        100000 + cat_idx * 10000 + grade_idx * 1000 + clarity_idx * 100 + lvl * 10
    }

    /// Dekoduje kanoniczny VNUM smoczego kamienia
    pub fn parse_vnum(vnum: u32) -> Option<(StoneCategory, DragonSoulGrade, DragonSoulClarity, u8)> {
        if !(110000..=170000).contains(&vnum) {
            return None;
        }
        let rem = vnum - 100000;
        let cat_idx = rem / 10000;
        let grade_idx = (rem % 10000) / 1000;
        let clarity_idx = (rem % 1000) / 100;
        let level = ((rem % 100) / 10) as u8;

        let category = match cat_idx {
            1 => StoneCategory::Diamond,
            2 => StoneCategory::Ruby,
            3 => StoneCategory::Jade,
            4 => StoneCategory::Sapphire,
            5 => StoneCategory::Garnet,
            6 => StoneCategory::Onyx,
            _ => return None,
        };
        let grade = match grade_idx {
            0 => DragonSoulGrade::Normal,
            1 => DragonSoulGrade::Brilliant,
            2 => DragonSoulGrade::Rare,
            3 => DragonSoulGrade::Ancient,
            4 => DragonSoulGrade::Legendary,
            _ => return None,
        };
        let clarity = match clarity_idx {
            0 => DragonSoulClarity::Lowest,
            1 => DragonSoulClarity::Low,
            2 => DragonSoulClarity::Mid,
            3 => DragonSoulClarity::High,
            4 => DragonSoulClarity::Highest,
            _ => return None,
        };
        Some((category, grade, clarity, level.min(DS_MAX_REFINE_LEVEL)))
    }

    /// Uszlachetnianie jakosci (Grade):
    /// Wymaga co najmniej 2 kamieni tej samej kategorii i jakosci.
    pub fn refine_grade(
        stones: &[DragonSoulRefineStone],
        success: bool,
    ) -> Result<DragonSoulRefineStone, DragonSoulRefineError> {
        if stones.len() < 2 {
            return Err(DragonSoulRefineError::NotEnoughStones);
        }
        let first = &stones[0];
        for stone in stones {
            if stone.category != first.category {
                return Err(DragonSoulRefineError::MismatchedCategory);
            }
            if stone.grade != first.grade {
                return Err(DragonSoulRefineError::MismatchedGrade);
            }
        }
        let next_grade = first.grade.next().ok_or(DragonSoulRefineError::MaxGradeReached)?;

        if success {
            Ok(DragonSoulRefineStone {
                category: first.category,
                grade: next_grade,
                clarity: DragonSoulClarity::Lowest,
                level: 0,
                bonus: Bonus {
                    attack: first.bonus.attack + 5,
                    defense: first.bonus.defense + 5,
                },
            })
        } else {
            Err(DragonSoulRefineError::RefineFailed)
        }
    }

    /// Uszlachetnianie stopnia czystosci (Clarity):
    /// Wymaga kamieni tej samej kategorii, jakosci i stopnia czystosci.
    pub fn refine_clarity(
        stones: &[DragonSoulRefineStone],
        success: bool,
    ) -> Result<DragonSoulRefineStone, DragonSoulRefineError> {
        if stones.len() < 2 {
            return Err(DragonSoulRefineError::NotEnoughStones);
        }
        let first = &stones[0];
        for stone in stones {
            if stone.category != first.category {
                return Err(DragonSoulRefineError::MismatchedCategory);
            }
            if stone.grade != first.grade {
                return Err(DragonSoulRefineError::MismatchedGrade);
            }
            if stone.clarity != first.clarity {
                return Err(DragonSoulRefineError::MismatchedClarity);
            }
        }
        let next_clarity = first.clarity.next().ok_or(DragonSoulRefineError::MaxClarityReached)?;

        if success {
            Ok(DragonSoulRefineStone {
                category: first.category,
                grade: first.grade,
                clarity: next_clarity,
                level: 0,
                bonus: Bonus {
                    attack: first.bonus.attack + 3,
                    defense: first.bonus.defense + 3,
                },
            })
        } else {
            Err(DragonSoulRefineError::RefineFailed)
        }
    }

    /// Uszlachetnianie poziomu kamienia (+0..+6):
    /// Wymaga smoczego ziarna (Dragon Soul Bean: np. 70251, 70252, 70253).
    pub fn refine_level(
        stone: &DragonSoulRefineStone,
        bean_vnum: u32,
        success: bool,
    ) -> Result<DragonSoulRefineStone, DragonSoulRefineError> {
        match bean_vnum {
            70251 | 70252 | 70253 => {},
            _ => return Err(DragonSoulRefineError::InvalidBeanItem),
        }

        if stone.level >= DS_MAX_REFINE_LEVEL {
            return Err(DragonSoulRefineError::MaxLevelReached);
        }

        if success {
            let next_level = stone.level + 1;
            Ok(DragonSoulRefineStone {
                category: stone.category,
                grade: stone.grade,
                clarity: stone.clarity,
                level: next_level,
                bonus: Bonus {
                    attack: stone.bonus.attack + 4,
                    defense: stone.bonus.defense + 4,
                },
            })
        } else {
            Err(DragonSoulRefineError::RefineFailed)
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DragonSoulDeck {
    pub diamond: Option<DragonSoulStone>,
    pub ruby: Option<DragonSoulStone>,
    pub jade: Option<DragonSoulStone>,
    pub sapphire: Option<DragonSoulStone>,
    pub garnet: Option<DragonSoulStone>,
    pub onyx: Option<DragonSoulStone>,
}

impl DragonSoulDeck {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn equip(&mut self, stone: DragonSoulStone) -> Option<DragonSoulStone> {
        match stone.category {
            StoneCategory::Diamond => std::mem::replace(&mut self.diamond, Some(stone)),
            StoneCategory::Ruby => std::mem::replace(&mut self.ruby, Some(stone)),
            StoneCategory::Jade => std::mem::replace(&mut self.jade, Some(stone)),
            StoneCategory::Sapphire => std::mem::replace(&mut self.sapphire, Some(stone)),
            StoneCategory::Garnet => std::mem::replace(&mut self.garnet, Some(stone)),
            StoneCategory::Onyx => std::mem::replace(&mut self.onyx, Some(stone)),
        }
    }

    pub fn unequip(&mut self, category: StoneCategory) -> Option<DragonSoulStone> {
        match category {
            StoneCategory::Diamond => self.diamond.take(),
            StoneCategory::Ruby => self.ruby.take(),
            StoneCategory::Jade => self.jade.take(),
            StoneCategory::Sapphire => self.sapphire.take(),
            StoneCategory::Garnet => self.garnet.take(),
            StoneCategory::Onyx => self.onyx.take(),
        }
    }

    pub fn get_set_grade(&self) -> Option<DragonSoulGrade> {
        let stones = [
            self.diamond.as_ref(),
            self.ruby.as_ref(),
            self.jade.as_ref(),
            self.sapphire.as_ref(),
            self.garnet.as_ref(),
            self.onyx.as_ref(),
        ];

        let mut current_grade = None;
        for stone_opt in stones.iter() {
            if let Some(stone) = stone_opt {
                if let Some(grade) = current_grade {
                    if grade != stone.grade {
                        return None;
                    }
                } else {
                    current_grade = Some(stone.grade);
                }
            } else {
                return None;
            }
        }
        current_grade
    }

    pub fn calculate_total_bonus(&self) -> Bonus {
        let mut total = Bonus::default();

        let stones = [
            self.diamond.as_ref(),
            self.ruby.as_ref(),
            self.jade.as_ref(),
            self.sapphire.as_ref(),
            self.garnet.as_ref(),
            self.onyx.as_ref(),
        ];

        for stone_opt in stones.iter() {
            if let Some(stone) = stone_opt {
                total = total.add(&stone.bonus);
            }
        }

        if let Some(set_grade) = self.get_set_grade() {
            let set_bonus = match set_grade {
                DragonSoulGrade::Normal => Bonus { attack: 10, defense: 10 },
                DragonSoulGrade::Brilliant => Bonus { attack: 20, defense: 20 },
                DragonSoulGrade::Rare => Bonus { attack: 30, defense: 30 },
                DragonSoulGrade::Ancient => Bonus { attack: 40, defense: 40 },
                DragonSoulGrade::Legendary => Bonus { attack: 50, defense: 50 },
            };
            total = total.add(&set_bonus);
        }

        total
    }
}

pub struct DragonSoulSystemV2 {
    pub deck1: DragonSoulDeck,
    pub deck2: DragonSoulDeck,
    pub active_deck: Option<DeckType>,
}

impl DragonSoulSystemV2 {
    pub fn new() -> Self {
        Self {
            deck1: DragonSoulDeck::new(),
            deck2: DragonSoulDeck::new(),
            active_deck: None,
        }
    }

    pub fn activate_deck(&mut self, deck: DeckType) {
        if self.active_deck == Some(deck) {
            self.active_deck = None;
        } else {
            self.active_deck = Some(deck);
        }
    }

    pub fn equip_stone(&mut self, deck: DeckType, stone: DragonSoulStone) -> Option<DragonSoulStone> {
        match deck {
            DeckType::Deck1 => self.deck1.equip(stone),
            DeckType::Deck2 => self.deck2.equip(stone),
        }
    }

    pub fn unequip_stone(&mut self, deck: DeckType, category: StoneCategory) -> Option<DragonSoulStone> {
        match deck {
            DeckType::Deck1 => self.deck1.unequip(category),
            DeckType::Deck2 => self.deck2.unequip(category),
        }
    }

    pub fn get_active_bonus(&self) -> Bonus {
        match self.active_deck {
            Some(DeckType::Deck1) => self.deck1.calculate_total_bonus(),
            Some(DeckType::Deck2) => self.deck2.calculate_total_bonus(),
            None => Bonus::default(),
        }
    }
}

impl Default for DragonSoulSystemV2 {
    fn default() -> Self {
        Self::new()
    }
}

static DRAGON_SOUL_SYSTEM: OnceLock<Mutex<DragonSoulSystemV2>> = OnceLock::new();

pub fn get_dragon_soul_system() -> &'static Mutex<DragonSoulSystemV2> {
    DRAGON_SOUL_SYSTEM.get_or_init(|| Mutex::new(DragonSoulSystemV2::new()))
}

#[no_mangle]
pub extern "C" fn m2_ds_activate_deck(deck_idx: u8) -> bool {
    let result = catch_unwind(|| {
        let deck_type = match deck_idx {
            0 => DeckType::Deck1,
            1 => DeckType::Deck2,
            _ => return false,
        };

        if let Ok(mut system) = get_dragon_soul_system().lock() {
            system.activate_deck(deck_type);
            true
        } else {
            false
        }
    });

    result.unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dragon_soul_vnum_roundtrip() {
        let vnum = DragonSoulRefine::make_vnum(
            StoneCategory::Ruby,
            DragonSoulGrade::Ancient,
            DragonSoulClarity::Mid,
            4,
        );
        let parsed = DragonSoulRefine::parse_vnum(vnum).unwrap();
        assert_eq!(parsed.0, StoneCategory::Ruby);
        assert_eq!(parsed.1, DragonSoulGrade::Ancient);
        assert_eq!(parsed.2, DragonSoulClarity::Mid);
        assert_eq!(parsed.3, 4);
    }

    #[test]
    fn test_dragon_soul_refine_grade() {
        let stone1 = DragonSoulRefineStone::new(
            StoneCategory::Diamond,
            DragonSoulGrade::Normal,
            DragonSoulClarity::Lowest,
            0,
            Bonus { attack: 10, defense: 10 },
        );
        let stone2 = stone1.clone();

        let upgraded = DragonSoulRefine::refine_grade(&[stone1, stone2], true).unwrap();
        assert_eq!(upgraded.grade, DragonSoulGrade::Brilliant);
        assert_eq!(upgraded.bonus.attack, 15);
    }

    #[test]
    fn test_dragon_soul_refine_clarity() {
        let stone1 = DragonSoulRefineStone::new(
            StoneCategory::Jade,
            DragonSoulGrade::Rare,
            DragonSoulClarity::Lowest,
            0,
            Bonus { attack: 10, defense: 10 },
        );
        let stone2 = stone1.clone();

        let upgraded = DragonSoulRefine::refine_clarity(&[stone1, stone2], true).unwrap();
        assert_eq!(upgraded.clarity, DragonSoulClarity::Low);
        assert_eq!(upgraded.bonus.attack, 13);
    }

    #[test]
    fn test_dragon_soul_refine_level() {
        let stone = DragonSoulRefineStone::new(
            StoneCategory::Sapphire,
            DragonSoulGrade::Rare,
            DragonSoulClarity::High,
            2,
            Bonus { attack: 20, defense: 20 },
        );

        // Uszlachetnienie zielonym ziarnem 70251 z sukcesem
        let upgraded = DragonSoulRefine::refine_level(&stone, 70251, true).unwrap();
        assert_eq!(upgraded.level, 3);
        assert_eq!(upgraded.bonus.attack, 24);

        // Max poziom +6
        let max_stone = DragonSoulRefineStone::new(
            StoneCategory::Sapphire,
            DragonSoulGrade::Rare,
            DragonSoulClarity::High,
            6,
            Bonus { attack: 30, defense: 30 },
        );
        assert_eq!(
            DragonSoulRefine::refine_level(&max_stone, 70251, true),
            Err(DragonSoulRefineError::MaxLevelReached)
        );
    }
}
