use std::collections::HashMap;

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
    pub fn add_or_update_actor(&mut self, vid: u32, x: f32, y: f32) {
        let actor = Actor { vid, x, y };
        self.actors.insert(vid, actor);
    }

    /// Usuwa aktora na podstawie podanego VID. 
    /// Zwraca true jesli aktor zostal usuniety, false w przeciwnym razie.
    pub fn remove_actor(&mut self, vid: u32) -> bool {
        self.actors.remove(&vid).is_some()
    }

    /// Zwraca referencje do aktora o podanym VID.
    /// Bezpiecznie obsluguje niepoprawne lub nieistniejace VID zwracajac Option.
    pub fn get_actor(&self, vid: u32) -> Option<&Actor> {
        self.actors.get(&vid)
    }

    /// Znajduje wszystkich aktorow znajdujacych sie w promieniu r od srodka (x, y).
    pub fn find_actors_in_radius(&self, x: f32, y: f32, radius: f32) -> Vec<&Actor> {
        let mut result = Vec::new();
        let radius_sq = radius * radius;

        for actor in self.actors.values() {
            let dx = actor.x - x;
            let dy = actor.y - y;
            let dist_sq = dx * dx + dy * dy;

            if dist_sq <= radius_sq {
                result.push(actor);
            }
        }

        result
    }
}
