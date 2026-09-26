#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StoneCategory {
    Diament,
    Rubin,
    Jadeit,
    Szafir,
    Granat,
    Onyks,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Deck {
    Deck1,
    Deck2,
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Bonus {
    pub hp: i32,
    pub sp: i32,
    pub attack: i32,
    pub defense: i32,
    pub magic_attack: i32,
    pub magic_defense: i32,
}

impl Bonus {
    pub fn add(&self, other: &Bonus) -> Bonus {
        Bonus {
            hp: self.hp + other.hp,
            sp: self.sp + other.sp,
            attack: self.attack + other.attack,
            defense: self.defense + other.defense,
            magic_attack: self.magic_attack + other.magic_attack,
            magic_defense: self.magic_defense + other.magic_defense,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DragonSoulStone {
    pub category: StoneCategory,
    pub vnum: u32,
    pub bonus: Bonus,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DragonSoulDeck {
    pub diament: Option<DragonSoulStone>,
    pub rubin: Option<DragonSoulStone>,
    pub jadeit: Option<DragonSoulStone>,
    pub szafir: Option<DragonSoulStone>,
    pub granat: Option<DragonSoulStone>,
    pub onyks: Option<DragonSoulStone>,
}

impl DragonSoulDeck {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn equip(&mut self, stone: DragonSoulStone) -> Option<DragonSoulStone> {
        match stone.category {
            StoneCategory::Diament => std::mem::replace(&mut self.diament, Some(stone)),
            StoneCategory::Rubin => std::mem::replace(&mut self.rubin, Some(stone)),
            StoneCategory::Jadeit => std::mem::replace(&mut self.jadeit, Some(stone)),
            StoneCategory::Szafir => std::mem::replace(&mut self.szafir, Some(stone)),
            StoneCategory::Granat => std::mem::replace(&mut self.granat, Some(stone)),
            StoneCategory::Onyks => std::mem::replace(&mut self.onyks, Some(stone)),
        }
    }

    pub fn unequip(&mut self, category: StoneCategory) -> Option<DragonSoulStone> {
        match category {
            StoneCategory::Diament => self.diament.take(),
            StoneCategory::Rubin => self.rubin.take(),
            StoneCategory::Jadeit => self.jadeit.take(),
            StoneCategory::Szafir => self.szafir.take(),
            StoneCategory::Granat => self.granat.take(),
            StoneCategory::Onyks => self.onyks.take(),
        }
    }

    pub fn calculate_total_bonus(&self) -> Bonus {
        let mut total = Bonus::default();

        if let Some(ref stone) = self.diament {
            total = total.add(&stone.bonus);
        }
        if let Some(ref stone) = self.rubin {
            total = total.add(&stone.bonus);
        }
        if let Some(ref stone) = self.jadeit {
            total = total.add(&stone.bonus);
        }
        if let Some(ref stone) = self.szafir {
            total = total.add(&stone.bonus);
        }
        if let Some(ref stone) = self.granat {
            total = total.add(&stone.bonus);
        }
        if let Some(ref stone) = self.onyks {
            total = total.add(&stone.bonus);
        }

        total
    }
}

#[derive(Debug, Clone)]
pub struct DragonSoulSystem {
    pub deck1: DragonSoulDeck,
    pub deck2: DragonSoulDeck,
    pub active_deck: Deck,
    pub is_dragon_soul_active: bool,
}

impl DragonSoulSystem {
    pub fn new() -> Self {
        Self {
            deck1: DragonSoulDeck::new(),
            deck2: DragonSoulDeck::new(),
            active_deck: Deck::Deck1,
            is_dragon_soul_active: false,
        }
    }

    pub fn switch_deck(&mut self, new_deck: Deck) {
        self.active_deck = new_deck;
    }

    pub fn set_active(&mut self, active: bool) {
        self.is_dragon_soul_active = active;
    }

    pub fn equip_stone(&mut self, deck: Deck, stone: DragonSoulStone) -> Option<DragonSoulStone> {
        match deck {
            Deck::Deck1 => self.deck1.equip(stone),
            Deck::Deck2 => self.deck2.equip(stone),
        }
    }

    pub fn unequip_stone(&mut self, deck: Deck, category: StoneCategory) -> Option<DragonSoulStone> {
        match deck {
            Deck::Deck1 => self.deck1.unequip(category),
            Deck::Deck2 => self.deck2.unequip(category),
        }
    }

    // Metoda zwracajaca bonus z aktywnej talii.
    // Zwraca wartosci tylko wtedy gdy system alchemii jest wlaczony.
    // W przeciwnym razie zwraca domyslny zerowy bonus.
    pub fn get_active_bonus(&self) -> Bonus {
        if !self.is_dragon_soul_active {
            return Bonus::default();
        }

        match self.active_deck {
            Deck::Deck1 => self.deck1.calculate_total_bonus(),
            Deck::Deck2 => self.deck2.calculate_total_bonus(),
        }
    }
}
