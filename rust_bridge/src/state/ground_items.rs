use std::collections::HashMap;
use std::sync::{OnceLock, RwLock};
use crate::ffi_guard::safe_ffi_call;

/// Reprezentuje przedmiot lezacy na ziemi w zasiegu gracza.
#[derive(Debug, Clone, PartialEq)]
pub struct GroundItem {
    pub vid: u32,
    pub vnum: u32,
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

/// Zarzadca przedmiotow lezacych na ziemi, oparty na VID.
pub struct GroundItemsCache {
    items: HashMap<u32, GroundItem>,
}

impl GroundItemsCache {
    /// Tworzy nowego zarzadce przedmiotow na ziemi.
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }

    /// Globalna instancja menedzera z zabezpieczeniem watkowym
    pub fn instance() -> &'static RwLock<GroundItemsCache> {
        static INSTANCE: OnceLock<RwLock<GroundItemsCache>> = OnceLock::new();
        INSTANCE.get_or_init(|| RwLock::new(GroundItemsCache::new()))
    }

    /// Dodaje nowy przedmiot do menedzera. Jesli przedmiot o danym VID juz istnieje, zostanie zaktualizowany.
    pub fn add_item(&mut self, item: GroundItem) {
        self.items.insert(item.vid, item);
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

    /// Czysci wszystkie przedmioty z pamieci menedzera.
    pub fn clear_items(&mut self) {
        self.items.clear();
    }

    /// Znajduje VID najblizszego przedmiotu do punktu (px, py).
    pub fn find_nearest(&self, px: i32, py: i32) -> Option<u32> {
        let mut nearest_vid = None;
        let mut min_dist_sq = i64::MAX;

        for item in self.items.values() {
            let dx = (item.x as i64) - (px as i64);
            let dy = (item.y as i64) - (py as i64);
            let dist_sq = dx * dx + dy * dy;

            if dist_sq < min_dist_sq {
                min_dist_sq = dist_sq;
                nearest_vid = Some(item.vid);
            }
        }

        nearest_vid
    }
}

impl Default for GroundItemsCache {
    fn default() -> Self {
        Self::new()
    }
}

/// Faza A: Zwraca VID najblizszego przedmiotu lezacego na ziemi dla podanej pozycji (px, py).
/// Zwraca 0, jesli brak przedmiotu.
#[no_mangle]
pub extern "C" fn m2_ground_find_nearest(px: i32, py: i32) -> u32 {
    safe_ffi_call(0, || {
        let cache = GroundItemsCache::instance().read().unwrap();
        cache.find_nearest(px, py).unwrap_or(0)
    })
}
