//! Modul mod_chrmgr dla skrzynki m2-client-pyapi
//! Modul Pythona chrmgr: menedzer postaci i mobow
//! Metody chrmgr.GetVIDName, chrmgr.GetRace, chrmgr.SelectCharacter

use std::collections::HashMap;

/// Reprezentacja postaci w grze
#[derive(Debug, Clone)]
pub struct Character {
    pub vid: u32,
    pub name: String,
    pub race: u32,
}

/// Glowna struktura implementacyjna menedzera postaci
#[derive(Debug, Clone, Default)]
pub struct ModChrmgr {
    pub initialized: bool,
    characters: HashMap<u32, Character>,
    selected_vid: Option<u32>,
}

impl ModChrmgr {
    /// Tworzy nowa instancje menedzera postaci
    pub fn new() -> Self {
        Self {
            initialized: true,
            characters: HashMap::new(),
            selected_vid: None,
        }
    }

    /// Sprawdza, czy menedzer zostal zainicjowany
    pub fn is_ready(&self) -> bool {
        self.initialized
    }

    /// Dodaje lub aktualizuje postac w menedzerze
    pub fn add_character(&mut self, vid: u32, name: String, race: u32) {
        self.characters.insert(vid, Character { vid, name, race });
    }

    /// Usuwa postac z menedzera na podstawie VID
    pub fn remove_character(&mut self, vid: u32) {
        self.characters.remove(&vid);
        if let Some(selected) = self.selected_vid {
            if selected == vid {
                self.selected_vid = None;
            }
        }
    }

    /// Zwraca nazwe postaci na podstawie podanego VID (chrmgr.GetVIDName)
    pub fn get_vid_name(&self, vid: u32) -> Option<String> {
        self.characters.get(&vid).map(|c| c.name.clone())
    }

    /// Zwraca rase postaci na podstawie podanego VID (chrmgr.GetRace)
    pub fn get_race(&self, vid: u32) -> Option<u32> {
        self.characters.get(&vid).map(|c| c.race)
    }

    /// Wybiera postac o podanym VID jako aktywna (chrmgr.SelectCharacter)
    pub fn select_character(&mut self, vid: u32) -> bool {
        if self.characters.contains_key(&vid) {
            self.selected_vid = Some(vid);
            true
        } else {
            false
        }
    }
    
    /// Zwraca VID aktualnie wybranej postaci
    pub fn get_selected_character(&self) -> Option<u32> {
        self.selected_vid
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skeleton_mod_chrmgr() {
        let obj = ModChrmgr::new();
        assert!(obj.is_ready());
    }
}
