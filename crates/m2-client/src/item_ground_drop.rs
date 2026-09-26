use std::collections::HashMap;

/// Reprezentacja przedmiotu upuszczonego na ziemie w swiecie gry.
#[derive(Debug, Clone)]
pub struct GroundItem {
    pub vid: u32,
    pub vnum: u32,
    pub count: u8,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub owner_name: Option<String>,
    pub drop_time_ms: u32,
}

/// Wynik proby podniesienia przedmiotu
#[derive(Debug, PartialEq, Eq)]
pub enum PickupResult {
    /// Podniesienie udane
    Success,
    /// Przedmiot jest zablokowany przez innego wlasciciela (ochrona 30 sekund)
    Protected(String),
    /// Przedmiot nie istnieje na ziemi
    NotFound,
}

pub struct ItemGroundDropManager {
    items: HashMap<u32, GroundItem>,
}

impl ItemGroundDropManager {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }

    /// Dodaje nowy upuszczony przedmiot na ziemie
    pub fn drop_item(&mut self, item: GroundItem) {
        self.items.insert(item.vid, item);
    }

    /// Probuje podniesc przedmiot o danym vid przez gracza o nazwie player_name.
    /// current_time_ms - obecny czas w milisekundach.
    /// Zgodnie z wymaganiami:
    /// - 0..30s: Tylko owner moze podniesc (jezeli owner nie jest ustawiony, kazdy moze).
    /// - > 30s: Kazdy moze podniesc.
    pub fn try_pickup(&mut self, vid: u32, player_name: &str, current_time_ms: u32) -> PickupResult {
        if let Some(item) = self.items.get(&vid) {
            let elapsed = current_time_ms.wrapping_sub(item.drop_time_ms);
            let is_protected = elapsed < 30_000;

            if is_protected {
                if let Some(ref owner) = item.owner_name {
                    if owner != player_name {
                        return PickupResult::Protected(owner.clone());
                    }
                }
            }
            
            // Jesli doszlismy tutaj, to mozemy podniesc (zdejmujemy z mapy)
            self.items.remove(&vid);
            PickupResult::Success
        } else {
            PickupResult::NotFound
        }
    }

    /// Aktualizuje stan przedmiotow - usuwa te, ktore leza na ziemi ponad 180 sekund.
    /// current_time_ms - obecny czas w milisekundach.
    pub fn update(&mut self, current_time_ms: u32) {
        self.items.retain(|_, item| {
            let elapsed = current_time_ms.wrapping_sub(item.drop_time_ms);
            elapsed <= 180_000
        });
    }

    /// Pobiera informacje o przedmiocie (uzyteczne do renderowania itp).
    pub fn get_item(&self, vid: u32) -> Option<&GroundItem> {
        self.items.get(&vid)
    }

    /// Czysci wszystkie przedmioty na ziemi
    pub fn clear(&mut self) {
        self.items.clear();
    }
}
