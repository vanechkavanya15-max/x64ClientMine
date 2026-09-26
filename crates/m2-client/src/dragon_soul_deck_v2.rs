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
                        return None; // Not all stones have the same grade
                    }
                } else {
                    current_grade = Some(stone.grade);
                }
            } else {
                return None; // Missing a stone, so no full set
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

        // Apply set bonus if all 6 stones are present and of the same grade
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

