use std::collections::HashMap;

/// Reprezentuje przedmiot lezacy na ziemi w zasiegu gracza.
#[derive(Debug, Clone, PartialEq)]
pub struct GroundItem {
    pub vid: u32,
    pub vnum: u32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub owner: u32,
}

/// Zarzadca przedmiotow lezacych na ziemi, oparty na VID.
pub struct GroundItemManager {
    items: HashMap<u32, GroundItem>,
}

impl GroundItemManager {
    /// Tworzy nowego zarzadce przedmiotow na ziemi.
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }

    /// Dodaje nowy przedmiot do menedzera. Jesli przedmiot o danym VID juz istnieje, zostanie zaktualizowany.
    pub fn add_item(&mut self, vid: u32, vnum: u32, x: f32, y: f32, z: f32, owner: u32) {
        let item = GroundItem {
            vid,
            vnum,
            x,
            y,
            z,
            owner,
        };
        self.items.insert(vid, item);
    }

    /// Usuwa przedmiot o podanym VID. Zwraca true jesli przedmiot istnial i zostal usuniety, false w przeciwnym razie.
    pub fn remove_item(&mut self, vid: u32) -> bool {
        self.items.remove(&vid).is_some()
    }

    /// Zwraca referencje do przedmiotu o podanym VID, jesli istnieje.
    pub fn get_item(&self, vid: u32) -> Option<&GroundItem> {
        self.items.get(&vid)
    }

    /// Zwraca wszystkie przedmioty, do ktorych mamy dostep.
    pub fn get_all_items(&self) -> Vec<&GroundItem> {
        self.items.values().collect()
    }

    /// Aktualizuje wlasciciela przedmiotu o podanym VID. Zwraca true jesli przedmiot istnial, false jesli nie.
    pub fn update_item_owner(&mut self, vid: u32, new_owner: u32) -> bool {
        if let Some(item) = self.items.get_mut(&vid) {
            item.owner = new_owner;
            true
        } else {
            false
        }
    }

    /// Cysci wszystkie przedmioty z pamieci menedzera.
    pub fn clear_items(&mut self) {
        self.items.clear();
    }
}

impl Default for GroundItemManager {
    fn default() -> Self {
        Self::new()
    }
}
