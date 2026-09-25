use std::collections::HashMap;
use std::sync::Mutex;
use once_cell::sync::Lazy;
use crate::ffi_guard::safe_ffi_call;

pub const ACTOR_MANAGER: u32 = 0;

/// Struktura reprezentujaca aktora w swiecie gry.
#[derive(Debug, Clone, PartialEq)]
pub struct Actor {
    pub vid: u32,
    pub x: f32,
    pub y: f32,
}

/// Menedzer aktorow zarzadzajacy ich stanem w pamieci.
#[derive(Debug, Default)]
pub struct ActorManager {
    actors: HashMap<u32, Actor>,
}

impl ActorManager {
    /// Tworzy nowego menedzera aktorow.
    pub fn new() -> Self {
        Self {
            actors: HashMap::new(),
        }
    }

    /// Dodaje nowego aktora lub aktualizuje pozycje istniejacego.
    pub fn insert(&mut self, actor: Actor) {
        self.actors.insert(actor.vid, actor);
    }

    /// Usuwa aktora na podstawie podanego VID.
    pub fn remove(&mut self, vid: u32) {
        self.actors.remove(&vid);
    }

    /// Zwraca referencje do aktora o podanym VID.
    /// Bezpiecznie obsluguje niepoprawne lub nieistniejace VID zwracajac Option.
    pub fn get(&self, vid: u32) -> Option<&Actor> {
        self.actors.get(&vid)
    }

    /// Znajduje najblizszego przeciwnika wzgledem podanych koordynatow.
    pub fn find_nearest_enemy(&self, from_x: i32, from_y: i32) -> Option<u32> {
        let from_x = from_x as f32;
        let from_y = from_y as f32;
        
        let mut nearest_vid = None;
        let mut min_dist_sq = f32::MAX;

        for actor in self.actors.values() {
            let dx = actor.x - from_x;
            let dy = actor.y - from_y;
            let dist_sq = dx * dx + dy * dy;

            if dist_sq < min_dist_sq {
                min_dist_sq = dist_sq;
                nearest_vid = Some(actor.vid);
            }
        }

        nearest_vid
    }

    /// Zwraca ilosc aktorow.
    pub fn count(&self) -> u32 {
        self.actors.len() as u32
    }
}

pub static GLOBAL_ACTOR_MANAGER: Lazy<Mutex<ActorManager>> = Lazy::new(|| Mutex::new(ActorManager::new()));

#[no_mangle]
pub unsafe extern "C" fn m2_actor_mgr_count() -> u32 {
    safe_ffi_call(0, || {
        if let Ok(mgr) = GLOBAL_ACTOR_MANAGER.lock() {
            mgr.count()
        } else {
            0
        }
    })
}

#[no_mangle]
pub unsafe extern "C" fn m2_actor_mgr_free(ptr: *mut ActorManager) {
    safe_ffi_call((), || {
        if !ptr.is_null() {
            let _ = Box::from_raw(ptr);
        }
    });
}
