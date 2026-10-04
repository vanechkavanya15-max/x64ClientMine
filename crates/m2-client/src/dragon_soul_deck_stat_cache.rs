use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub enum DragonSoulSlot {
    Diamond = 0,
    Ruby = 1,
    Jade = 2,
    Sapphire = 3,
    Garnet = 4,
    Onyx = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DragonSoulDeckType {
    Deck1,
    Deck2,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DragonSoulStats {
    pub attack: u32,
    pub defense: u32,
    pub hp: u32,
    pub sp: u32,
}

impl DragonSoulStats {
    pub fn add(&mut self, other: &Self) {
        self.attack += other.attack;
        self.defense += other.defense;
        self.hp += other.hp;
        self.sp += other.sp;
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DragonSoulItem {
    pub stats: DragonSoulStats,
    pub energy_left: u32, // remaining lifetime in seconds (or abstract ticks)
}

impl DragonSoulItem {
    pub fn is_active(&self) -> bool {
        self.energy_left > 0
    }
}

#[derive(Debug, Clone, Default)]
pub struct DragonSoulDeck {
    pub slots: [Option<DragonSoulItem>; 6],
}

impl DragonSoulDeck {
    pub fn equip(&mut self, slot: DragonSoulSlot, item: DragonSoulItem) -> Option<DragonSoulItem> {
        let prev = self.slots[slot as usize].take();
        self.slots[slot as usize] = Some(item);
        prev
    }

    pub fn unequip(&mut self, slot: DragonSoulSlot) -> Option<DragonSoulItem> {
        self.slots[slot as usize].take()
    }

    pub fn get_active_stats(&self) -> DragonSoulStats {
        let mut total = DragonSoulStats::default();
        for item_opt in self.slots.iter() {
            if let Some(item) = item_opt {
                if item.is_active() {
                    total.add(&item.stats);
                }
            }
        }
        total
    }

    pub fn tick_energy(&mut self, drain_amount: u32) {
        for item_opt in self.slots.iter_mut() {
            if let Some(item) = item_opt {
                if item.energy_left >= drain_amount {
                    item.energy_left -= drain_amount;
                } else {
                    item.energy_left = 0;
                }
            }
        }
    }
}

pub struct DragonSoulDeckStatCache {
    pub deck1: DragonSoulDeck,
    pub deck2: DragonSoulDeck,
    pub active_deck: Option<DragonSoulDeckType>,
    pub last_toggle_time: Option<Instant>,
    pub toggle_cooldown: Duration,
}

impl DragonSoulDeckStatCache {
    pub fn new(cooldown: Duration) -> Self {
        Self {
            deck1: DragonSoulDeck::default(),
            deck2: DragonSoulDeck::default(),
            active_deck: None,
            last_toggle_time: None,
            toggle_cooldown: cooldown,
        }
    }

    pub fn toggle_deck(&mut self, target_deck: DragonSoulDeckType, now: Instant) -> Result<(), &'static str> {
        if let Some(last_time) = self.last_toggle_time {
            if now.duration_since(last_time) < self.toggle_cooldown {
                return Err("Cooldown active");
            }
        }

        if self.active_deck == Some(target_deck) {
            self.active_deck = None; // turn off
        } else {
            self.active_deck = Some(target_deck);
        }
        
        self.last_toggle_time = Some(now);
        Ok(())
    }
    
    #[allow(dead_code)]
    pub fn deactivate_all(&mut self, now: Instant) -> Result<(), &'static str> {
        if let Some(last_time) = self.last_toggle_time {
            if now.duration_since(last_time) < self.toggle_cooldown {
                return Err("Cooldown active");
            }
        }
        
        self.active_deck = None;
        self.last_toggle_time = Some(now);
        Ok(())
    }

    pub fn get_active_stats(&self) -> DragonSoulStats {
        match self.active_deck {
            Some(DragonSoulDeckType::Deck1) => self.deck1.get_active_stats(),
            Some(DragonSoulDeckType::Deck2) => self.deck2.get_active_stats(),
            None => DragonSoulStats::default(),
        }
    }

    pub fn tick_active_deck_energy(&mut self, drain_amount: u32) {
        match self.active_deck {
            Some(DragonSoulDeckType::Deck1) => self.deck1.tick_energy(drain_amount),
            Some(DragonSoulDeckType::Deck2) => self.deck2.tick_energy(drain_amount),
            None => {}
        }
    }
}
