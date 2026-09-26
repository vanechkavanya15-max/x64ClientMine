use std::collections::HashMap;
use std::sync::{OnceLock, RwLock};
use std::panic::{catch_unwind, AssertUnwindSafe};

/// Struktura reprezentujaca aktora w grze
#[derive(Debug, Clone, Copy)]
pub struct Actor {
    pub vid: u32,
    pub race: u32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub rot: f32,
    pub time_alive_ms: u32,
}

/// Globalny stan przechowujacy aktorow, chroniony przez RwLock.
static ACTORS: OnceLock<RwLock<HashMap<u32, Actor>>> = OnceLock::new();

fn get_actors() -> &'static RwLock<HashMap<u32, Actor>> {
    ACTORS.get_or_init(|| RwLock::new(HashMap::new()))
}

/// Dodaje nowego aktora o podanym ID.
/// Zwraca true, jesli dodano pomyslnie.
/// Zwraca false, jesli aktor o takim VID juz istnieje.
#[no_mangle]
pub extern "C" fn m2_chr_create(vid: u32, race: u32, x: f32, y: f32, z: f32, rot: f32) -> bool {
    catch_unwind(AssertUnwindSafe(|| {
        let mut actors = get_actors().write().unwrap();
        if actors.contains_key(&vid) {
            return false;
        }
        actors.insert(vid, Actor {
            vid,
            race,
            x,
            y,
            z,
            rot,
            time_alive_ms: 0,
        });
        true
    })).unwrap_or(false)
}

/// Usuwa aktora o podanym ID.
/// Zwraca true, jesli usunieto pomyslnie.
/// Zwraca false, jesli aktora o takim VID nie znaleziono.
#[no_mangle]
pub extern "C" fn m2_chr_delete(vid: u32) -> bool {
    catch_unwind(AssertUnwindSafe(|| {
        let mut actors = get_actors().write().unwrap();
        actors.remove(&vid).is_some()
    })).unwrap_or(false)
}

/// Pobiera liczbe aktywnych aktorow.
#[no_mangle]
pub extern "C" fn m2_chr_get_count() -> u32 {
    catch_unwind(AssertUnwindSafe(|| {
        let actors = get_actors().read().unwrap();
        actors.len() as u32
    })).unwrap_or(0)
}

/// Aktualizuje stan w czasie dla wszystkich aktorow.
#[no_mangle]
pub extern "C" fn m2_chr_update(delta_time_ms: u32) {
    let _ = catch_unwind(AssertUnwindSafe(|| {
        let mut actors = get_actors().write().unwrap();
        for actor in actors.values_mut() {
            actor.time_alive_ms = actor.time_alive_ms.saturating_add(delta_time_ms);
        }
    }));
}

/// Funkcja pomocnicza dla testow, pozwalajaca na bezposredni dostep do aktora po VID
pub fn test_get_actor(vid: u32) -> Option<Actor> {
    let actors = get_actors().read().unwrap();
    actors.get(&vid).copied()
}

/// Funkcja pomocnicza dla testow, czyszczaca stan globalny
pub fn test_clear_actors() {
    let mut actors = get_actors().write().unwrap();
    actors.clear();
}

/// Funkcja pomocnicza, pozwalajaca na bezposredni dostep do wspolrzednych aktora po VID.
pub fn get_actor_pos(vid: u32) -> Option<(f32, f32, f32)> {
    let actors = get_actors().read().unwrap();
    actors.get(&vid).map(|a| (a.x, a.y, a.z))
}
