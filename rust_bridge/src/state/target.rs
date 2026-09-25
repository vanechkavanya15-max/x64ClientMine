use std::sync::RwLock;
use once_cell::sync::Lazy;

// Struktura przechowujaca informacje o celu
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Target {
    pub vid: u32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub is_alive: bool,
}

// Menedzer celu zarzadzajacy biezacym celem gracza
pub struct TargetManager {
    current_target: RwLock<Option<Target>>,
}

impl TargetManager {
    // Inicjalizuje nowy menedzer celu
    pub fn new() -> Self {
        Self {
            current_target: RwLock::new(None),
        }
    }

    // Ustawia nowy cel lub aktualizuje istniejacy
    // vid: Identyfikator celu, musi byc rozny od zera (0 oznacza bledny lub pusty VID)
    pub fn set_target(&self, vid: u32, x: f32, y: f32, z: f32, is_alive: bool) {
        if vid == 0 {
            // Ignorujemy bledny VID
            return;
        }

        let target = Target {
            vid,
            x,
            y,
            z,
            is_alive,
        };

        if let Ok(mut current) = self.current_target.write() {
            *current = Some(target);
        }
    }

    // Czysci aktualny cel (brak celu)
    pub fn clear_target(&self) {
        if let Ok(mut current) = self.current_target.write() {
            *current = None;
        }
    }

    // Oblicza odleglosc miedzy graczem a celem
    pub fn get_target_distance(&self, player_x: f32, player_y: f32, player_z: f32) -> Option<f32> {
        let current = self.current_target.read().ok()?;
        if let Some(target) = *current {
            let dx = target.x - player_x;
            let dy = target.y - player_y;
            let dz = target.z - player_z;
            let distance = (dx * dx + dy * dy + dz * dz).sqrt();
            Some(distance)
        } else {
            None
        }
    }

    // Sprawdza czy cel zyje
    pub fn is_target_alive(&self) -> bool {
        if let Ok(current) = self.current_target.read() {
            if let Some(target) = *current {
                return target.is_alive;
            }
        }
        false
    }

    // Sprawdza czy menedzer posiada ustawiony cel
    pub fn has_target(&self) -> bool {
        if let Ok(current) = self.current_target.read() {
            current.is_some()
        } else {
            false
        }
    }

    // Pobiera VID aktualnego celu
    pub fn get_target_vid(&self) -> Option<u32> {
        if let Ok(current) = self.current_target.read() {
            current.map(|t| t.vid)
        } else {
            None
        }
    }
}

impl Default for TargetManager {
    fn default() -> Self {
        Self::new()
    }
}

// Globalna instancja menedzera celu
pub static TARGET_MANAGER: Lazy<TargetManager> = Lazy::new(|| TargetManager::new());
