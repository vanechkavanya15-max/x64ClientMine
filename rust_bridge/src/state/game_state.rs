use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::RwLock;

use crate::ffi_guard::safe_ffi_call;

pub const POINT_HP: u8 = 5;
pub const POINT_MAX_HP: u8 = 6;
pub const POINT_SP: u8 = 7;
pub const POINT_MAX_SP: u8 = 8;
pub const POINT_GOLD: u8 = 11;

/// Struktura danych reprezentujaca przedmiot w ekwipunku lub oknie gry
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ItemData {
    pub vnum: u32,
    pub count: u8,
    pub flags: u32,
    pub anti_flags: u32,
    pub sockets: [i32; 3],
    pub attrs: [(u8, i16); 7],
}

impl Default for ItemData {
    fn default() -> Self {
        Self {
            vnum: 0,
            count: 0,
            flags: 0,
            anti_flags: 0,
            sockets: [0; 3],
            attrs: [(0, 0); 7],
        }
    }
}

impl ItemData {
    pub fn new(vnum: u32, count: u8) -> Self {
        Self {
            vnum,
            count,
            ..Default::default()
        }
    }
}

/// Alias kompatybilnosciowy dla typu ItemData
pub type GameStateItem = ItemData;

/// Pelny stan lokalnego gracza
#[derive(Debug, Clone)]
pub struct PlayerState {
    pub vid: u32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub rot: f32,
    pub points: [i64; 256],
    pub inventory: HashMap<(u8, u16), ItemData>,
    pub equipment: HashMap<u8, u32>,
    pub dragon_soul: HashMap<(u8, u8), u32>,
    pub mount_vnum: u32,
    pub is_mounted: bool,
}

impl Default for PlayerState {
    fn default() -> Self {
        Self {
            vid: 0,
            x: 0.0,
            y: 0.0,
            z: 0.0,
            rot: 0.0,
            points: [0; 256],
            inventory: HashMap::new(),
            equipment: HashMap::new(),
            dragon_soul: HashMap::new(),
            mount_vnum: 0,
            is_mounted: false,
        }
    }
}

/// Reprezentacja aktora (innego gracza, moba, NPC) w swiecie gry
#[derive(Debug, Clone, PartialEq)]
pub struct Actor {
    pub vid: u32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub rot: f32,
    pub race: u16,
    pub b_type: u8,
    pub hp: i32,
    pub max_hp: i32,
    pub name: String,
}

impl Default for Actor {
    fn default() -> Self {
        Self {
            vid: 0,
            x: 0.0,
            y: 0.0,
            z: 0.0,
            rot: 0.0,
            race: 0,
            b_type: 0,
            hp: 0,
            max_hp: 0,
            name: String::new(),
        }
    }
}

/// Alias kompatybilnosciowy dla typu Actor
pub type GameStateActor = Actor;

/// Centralny zarzadca stanu gry
#[derive(Debug, Clone, Default)]
pub struct GameState {
    pub player: PlayerState,
    pub actors: HashMap<u32, Actor>,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            player: PlayerState::default(),
            actors: HashMap::new(),
        }
    }

    pub fn reset(&mut self) {
        self.player = PlayerState::default();
        self.actors.clear();
    }

    /// Pobiera pozycje oraz rotacje gracza: (x, y, z, rot)
    pub fn get_player_pos(&self) -> (f32, f32, f32, f32) {
        (self.player.x, self.player.y, self.player.z, self.player.rot)
    }

    /// Ustawia pozycje oraz rotacje gracza
    pub fn set_player_pos(&mut self, x: f32, y: f32, z: f32, rot: f32) {
        self.player.x = x;
        self.player.y = y;
        self.player.z = z;
        self.player.rot = rot;
    }

    /// Pobiera wartosc punktu/statystyki gracza dla danego typu
    pub fn get_point(&self, point_type: u8) -> i64 {
        self.player.points[point_type as usize]
    }

    /// Ustawia wartosc punktu/statystyki gracza
    pub fn set_point(&mut self, point_type: u8, val: i64) {
        self.player.points[point_type as usize] = val;
    }

    /// Ustawia zbiorczo punkty z bufora pakietu punktow gracza
    pub fn set_points(&mut self, pts: &[i32; 255]) {
        for (i, &p) in pts.iter().enumerate() {
            if i < 256 {
                self.player.points[i] = p as i64;
            }
        }
    }

    /// Umieszcza przedmiot w danym oknie i slocie ekwipunku
    pub fn set_item(&mut self, window: u8, cell: u16, item: ItemData) {
        let vnum = item.vnum;
        self.player.inventory.insert((window, cell), item);
        self.player.equipment.insert(cell as u8, vnum);
        self.player.dragon_soul.insert((window, cell as u8), vnum);
    }

    /// Usuwa przedmiot z danego okna i slotu ekwipunku
    pub fn del_item(&mut self, window: u8, cell: u16) {
        self.player.inventory.remove(&(window, cell));
        self.player.equipment.remove(&(cell as u8));
        self.player.dragon_soul.remove(&(window, cell as u8));
    }

    /// Pobiera referencje do przedmiotu w danym oknie i komorce
    pub fn get_item(&self, window: u8, cell: u16) -> Option<&ItemData> {
        self.player.inventory.get(&(window, cell))
    }

    pub fn get_item_vnum(&self, window: u8, cell: u16) -> u32 {
        self.get_item(window, cell).map(|it| it.vnum).unwrap_or(0)
    }

    pub fn get_item_count(&self, window: u8, cell: u16) -> u32 {
        self.get_item(window, cell).map(|it| it.count as u32).unwrap_or(0)
    }

    pub fn get_hp(&self) -> i32 {
        self.player.points[POINT_HP as usize] as i32
    }

    pub fn get_max_hp(&self) -> i32 {
        self.player.points[POINT_MAX_HP as usize] as i32
    }

    pub fn get_sp(&self) -> i32 {
        self.player.points[POINT_SP as usize] as i32
    }

    pub fn get_max_sp(&self) -> i32 {
        self.player.points[POINT_MAX_SP as usize] as i32
    }

    pub fn get_gold(&self) -> i64 {
        self.player.points[POINT_GOLD as usize]
    }

    /// Dodaje nowego aktora lub aktualizuje istniejacego
    pub fn add_or_update_actor(&mut self, actor: Actor) {
        self.actors.insert(actor.vid, actor);
    }

    /// Usuwa aktora o podanym identyfikatorze VID
    pub fn remove_actor(&mut self, vid: u32) {
        self.actors.remove(&vid);
    }

    /// Pobiera referencje do aktora o podanym VID
    pub fn get_actor(&self, vid: u32) -> Option<&Actor> {
        self.actors.get(&vid)
    }

    /// Wyszukuje VID najblizszego aktora spelniajacego kryteria typu oraz zasiegu.
    /// Oblicza rzeczywisty dystans euklidesowy: sqrt(dx*dx + dy*dy) miedzy graczem a aktorem.
    /// Gdy actor_type == 0, dopasowuje dowolny typ.
    pub fn find_nearest_actor(&self, actor_type: u32, max_distance: f32) -> u32 {
        let mut nearest_vid = 0u32;
        let mut min_distance = if max_distance > 0.0 { max_distance } else { f32::MAX };

        for actor in self.actors.values() {
            if actor_type != 0 && (actor.b_type as u32) != actor_type {
                continue;
            }

            let dx = self.player.x - actor.x;
            let dy = self.player.y - actor.y;
            let dist = (dx * dx + dy * dy).sqrt();

            if dist <= min_distance {
                min_distance = dist;
                nearest_vid = actor.vid;
            }
        }

        nearest_vid
    }

    /// Oblicza rzeczywisty dystans euklidesowy: sqrt(dx*dx + dy*dy) miedzy graczem a aktorem o podanym VID.
    /// Zwraca 0.0 jesli aktor nie istnieje.
    pub fn actor_distance(&self, vid: u32) -> f32 {
        if let Some(actor) = self.actors.get(&vid) {
            let dx = self.player.x - actor.x;
            let dy = self.player.y - actor.y;
            (dx * dx + dy * dy).sqrt()
        } else {
            0.0
        }
    }

    /// Ustawia VID lokalnego gracza
    pub fn set_player_vid(&mut self, vid: u32) {
        self.player.vid = vid;
    }

    /// Pobiera VID lokalnego gracza
    pub fn get_player_vid(&self) -> u32 {
        self.player.vid
    }

    /// Aktualizuje pozycje aktora lub gracza
    pub fn update_actor_move(&mut self, vid: u32, x: i32, y: i32, rot: u8) {
        let rot_deg = rot as f32 * 5.0;
        let x_f = x as f32;
        let y_f = y as f32;

        if let Some(actor) = self.actors.get_mut(&vid) {
            actor.x = x_f;
            actor.y = y_f;
            actor.rot = rot_deg;
        }
        if self.player.vid == 0 || vid == self.player.vid {
            self.player.x = x_f;
            self.player.y = y_f;
            self.player.rot = rot_deg;
        }
    }

    /// Aplikuje obrazenia na aktorze lub graczu
    pub fn apply_damage(&mut self, victim_vid: u32, damage: i32) {
        if let Some(actor) = self.actors.get_mut(&victim_vid) {
            actor.hp = actor.hp.saturating_sub(damage).max(0);
        }
        if self.player.vid == 0 || victim_vid == self.player.vid {
            let hp = self.player.points[POINT_HP as usize];
            self.player.points[POINT_HP as usize] = (hp - damage as i64).max(0);
        }
    }
}

/// Globalna instancja GameState chroniona RwLock
pub static GLOBAL_GAME_STATE: Lazy<RwLock<GameState>> = Lazy::new(|| RwLock::new(GameState::new()));

#[no_mangle]
pub unsafe extern "C" fn m2_game_state_reset() {
    safe_ffi_call((), || {
        if let Ok(mut state) = GLOBAL_GAME_STATE.write() {
            state.reset();
        }
    });
}

#[no_mangle]
pub unsafe extern "C" fn m2_game_state_get_point(point_type: u8) -> i64 {
    safe_ffi_call(0, || {
        if let Ok(state) = GLOBAL_GAME_STATE.read() {
            state.get_point(point_type)
        } else {
            0
        }
    })
}

#[no_mangle]
pub unsafe extern "C" fn m2_game_state_get_player_vid() -> u32 {
    safe_ffi_call(0, || {
        if let Ok(state) = GLOBAL_GAME_STATE.read() {
            state.get_player_vid()
        } else {
            0
        }
    })
}

#[no_mangle]
pub unsafe extern "C" fn m2_game_state_actor_count() -> u32 {
    safe_ffi_call(0, || {
        if let Ok(state) = GLOBAL_GAME_STATE.read() {
            state.actors.len() as u32
        } else {
            0
        }
    })
}

#[no_mangle]
pub unsafe extern "C" fn m2_game_state_item_count() -> u32 {
    safe_ffi_call(0, || {
        if let Ok(state) = GLOBAL_GAME_STATE.read() {
            state.player.inventory.len() as u32
        } else {
            0
        }
    })
}
