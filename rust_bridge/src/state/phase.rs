// Reprezentuje fazy stanu polaczenia i gry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    Offline,
    Handshake,
    Login,
    Select,
    Game,
}

// Bledy zwiazane z zarzadzaniem stanem gry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StateError {
    InvalidVid(u32),
    IndexOutOfBounds(usize),
}

// Struktura reprezentujaca menedzer stanu gry.
#[derive(Debug, Clone)]
pub struct StateManager {
    current_phase: Phase,
    vids: Vec<u32>,
}

impl StateManager {
    // Tworzy nowy menedzer stanu poczatkowo w fazie Offline.
    pub fn new() -> Self {
        Self {
            current_phase: Phase::Offline,
            vids: Vec::new(),
        }
    }

    // Pobiera aktualna faze gry.
    pub fn get_phase(&self) -> Phase {
        self.current_phase
    }

    // Zmienia aktualna faze gry.
    pub fn set_phase(&mut self, phase: Phase) {
        self.current_phase = phase;
    }

    // Dodaje VID do menedzera stanu.
    pub fn add_vid(&mut self, vid: u32) -> Result<(), StateError> {
        if vid == 0 {
            return Err(StateError::InvalidVid(vid));
        }
        if !self.vids.contains(&vid) {
            self.vids.push(vid);
        }
        Ok(())
    }

    // Usuwa VID z menedzera stanu.
    pub fn remove_vid(&mut self, vid: u32) -> Result<(), StateError> {
        if vid == 0 {
            return Err(StateError::InvalidVid(vid));
        }
        if let Some(pos) = self.vids.iter().position(|&v| v == vid) {
            self.vids.remove(pos);
            Ok(())
        } else {
            Err(StateError::InvalidVid(vid))
        }
    }

    // Pobiera VID na podanym indeksie, zabezpieczone przed przekroczeniem zakresu.
    pub fn get_vid_at(&self, index: usize) -> Result<u32, StateError> {
        if index < self.vids.len() {
            Ok(self.vids[index])
        } else {
            Err(StateError::IndexOutOfBounds(index))
        }
    }

    // Sprawdza czy dany VID znajduje sie w menedzerze stanu.
    pub fn has_vid(&self, vid: u32) -> bool {
        self.vids.contains(&vid)
    }

    // Czysci wszystkie VIDs.
    pub fn clear_vids(&mut self) {
        self.vids.clear();
    }
}
