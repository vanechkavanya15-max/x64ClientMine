use std::collections::HashMap;

// Struktura reprezentujaca pojedynczego aktora w swiecie gry.
// Zawiera podstawowe informacje takie jak VID, koordynaty, rotacje, rase, HP i stan agresji.
#[derive(Debug, Clone, PartialEq)]
pub struct Actor {
    pub vid: u32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub rotation: f32,
    pub race: u32,
    pub hp: u32,
    pub is_aggressive: bool,
}

impl Actor {
    // Tworzy nowa instancje aktora
    pub fn new(vid: u32, x: f32, y: f32, z: f32, rotation: f32, race: u32, hp: u32, is_aggressive: bool) -> Self {
        Self {
            vid,
            x,
            y,
            z,
            rotation,
            race,
            hp,
            is_aggressive,
        }
    }
}

// Menedzer stanu aktorow
// Uzywa HashMap do przechowywania aktorow, co zapewnia bezpieczenstwo przed nieprawidlowymi VID
// i pozwala na bezpieczne operacje bez ryzyka wyjscia poza zakres tablicy.
pub struct ActorManager {
    actors: HashMap<u32, Actor>,
}

impl ActorManager {
    // Inicjalizuje pusty menedzer aktorow
    pub fn new() -> Self {
        Self {
            actors: HashMap::new(),
        }
    }

    // Dodaje nowego aktora lub aktualizuje istniejacego
    pub fn add_or_update_actor(&mut self, actor: Actor) {
        self.actors.insert(actor.vid, actor);
    }

    // Usuwa aktora na podstawie VID. Zwraca true jesli aktor zostal usuniety.
    pub fn remove_actor(&mut self, vid: u32) -> bool {
        self.actors.remove(&vid).is_some()
    }

    // Pobiera niemutowalna referencje do aktora o podanym VID.
    pub fn get_actor(&self, vid: u32) -> Option<&Actor> {
        self.actors.get(&vid)
    }

    // Pobiera mutowalna referencje do aktora o podanym VID.
    pub fn get_actor_mut(&mut self, vid: u32) -> Option<&mut Actor> {
        self.actors.get_mut(&vid)
    }

    // Aktualizuje pozycje aktora. Zwraca true jesli aktor istnieje i zostal zaktualizowany.
    pub fn update_position(&mut self, vid: u32, x: f32, y: f32, z: f32, rotation: f32) -> bool {
        if let Some(actor) = self.actors.get_mut(&vid) {
            actor.x = x;
            actor.y = y;
            actor.z = z;
            actor.rotation = rotation;
            true
        } else {
            false
        }
    }

    // Aktualizuje HP aktora. Zwraca true jesli aktor istnieje i zostal zaktualizowany.
    pub fn update_hp(&mut self, vid: u32, hp: u32) -> bool {
        if let Some(actor) = self.actors.get_mut(&vid) {
            actor.hp = hp;
            true
        } else {
            false
        }
    }

    // Zwraca ilosc aktualnie przechowywanych aktorow
    pub fn count(&self) -> usize {
        self.actors.len()
    }

    // Czysci wszystkich aktorow
    pub fn clear(&mut self) {
        self.actors.clear();
    }
}
