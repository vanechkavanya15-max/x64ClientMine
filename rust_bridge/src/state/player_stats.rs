use std::collections::HashMap;
use std::sync::{OnceLock, RwLock};

// Struktura przechowujaca statystyki postaci
#[derive(Debug, Clone, Default)]
pub struct PlayerStats {
    pub lvl: u32,
    pub exp: u64,
    pub max_exp: u64,
    pub hp: u32,
    pub mp: u32,
    pub str_points: u32, // Punkty statusu STR
    pub dex_points: u32, // Punkty statusu DEX
    pub int_points: u32, // Punkty statusu INT
    pub con_points: u32, // Punkty statusu CON
}

impl PlayerStats {
    // Inicjalizacja nowych statystyk
    pub fn new() -> Self {
        Self::default()
    }
}

// Menedzer statystyk graczy
pub struct PlayerStatsManager {
    // Mapa wirtualnych identyfikatorow (VID) na statystyki
    stats: HashMap<u32, PlayerStats>,
}

impl PlayerStatsManager {
    // Tworzenie nowej instancji menedzera
    pub fn new() -> Self {
        Self {
            stats: HashMap::new(),
        }
    }

    // Globalna instancja menedzera z zabezpieczeniem watkowym
    pub fn instance() -> &'static RwLock<PlayerStatsManager> {
        static INSTANCE: OnceLock<RwLock<PlayerStatsManager>> = OnceLock::new();
        INSTANCE.get_or_init(|| RwLock::new(PlayerStatsManager::new()))
    }

    // Rejestracja nowego gracza. Zwraca true jesli dodano, false jesli gracz juz istnial.
    pub fn create_player(&mut self, vid: u32) -> bool {
        if self.stats.contains_key(&vid) {
            false
        } else {
            self.stats.insert(vid, PlayerStats::new());
            true
        }
    }

    // Usuniecie gracza z menedzera
    pub fn remove_player(&mut self, vid: u32) -> bool {
        self.stats.remove(&vid).is_some()
    }

    // Pobranie kopii statystyk gracza. Odporne na bledne VID - zwraca Option.
    pub fn get_stats(&self, vid: u32) -> Option<PlayerStats> {
        self.stats.get(&vid).cloned()
    }

    // Bezpieczne ustawienie poziomu
    pub fn set_lvl(&mut self, vid: u32, lvl: u32) -> bool {
        if let Some(player) = self.stats.get_mut(&vid) {
            player.lvl = lvl;
            true
        } else {
            false
        }
    }

    // Bezpieczne dodanie punktow doswiadczenia z zabezpieczeniem przed przepelnieniem
    pub fn add_exp(&mut self, vid: u32, amount: u64) -> bool {
        if let Some(player) = self.stats.get_mut(&vid) {
            player.exp = player.exp.saturating_add(amount);
            // Limitujemy do max_exp jesli jest ustawione na wartosc wieksza niz 0
            if player.max_exp > 0 && player.exp > player.max_exp {
                player.exp = player.max_exp;
            }
            true
        } else {
            false
        }
    }

    // Ustawienie maksymalnego doswiadczenia
    pub fn set_max_exp(&mut self, vid: u32, max_exp: u64) -> bool {
        if let Some(player) = self.stats.get_mut(&vid) {
            player.max_exp = max_exp;
            if player.exp > player.max_exp && player.max_exp > 0 {
                player.exp = player.max_exp;
            }
            true
        } else {
            false
        }
    }

    // Bezpieczne dodanie HP
    pub fn add_hp(&mut self, vid: u32, amount: u32) -> bool {
        if let Some(player) = self.stats.get_mut(&vid) {
            player.hp = player.hp.saturating_add(amount);
            true
        } else {
            false
        }
    }

    // Bezpieczne odejmowanie HP (z zabezpieczeniem przed zejsciem ponizej zera)
    pub fn sub_hp(&mut self, vid: u32, amount: u32) -> bool {
        if let Some(player) = self.stats.get_mut(&vid) {
            player.hp = player.hp.saturating_sub(amount);
            true
        } else {
            false
        }
    }

    // Bezpieczne dodanie MP
    pub fn add_mp(&mut self, vid: u32, amount: u32) -> bool {
        if let Some(player) = self.stats.get_mut(&vid) {
            player.mp = player.mp.saturating_add(amount);
            true
        } else {
            false
        }
    }

    // Bezpieczne odejmowanie MP
    pub fn sub_mp(&mut self, vid: u32, amount: u32) -> bool {
        if let Some(player) = self.stats.get_mut(&vid) {
            player.mp = player.mp.saturating_sub(amount);
            true
        } else {
            false
        }
    }

    // Metoda do ustawienia wszystkich punktow statusu jednoczesnie
    pub fn set_status_points(&mut self, vid: u32, str_pts: u32, dex_pts: u32, int_pts: u32, con_pts: u32) -> bool {
        if let Some(player) = self.stats.get_mut(&vid) {
            player.str_points = str_pts;
            player.dex_points = dex_pts;
            player.int_points = int_pts;
            player.con_points = con_pts;
            true
        } else {
            false
        }
    }
    
    // Zwiekszenie sily
    pub fn add_str(&mut self, vid: u32, amount: u32) -> bool {
        if let Some(player) = self.stats.get_mut(&vid) {
            player.str_points = player.str_points.saturating_add(amount);
            true
        } else {
            false
        }
    }
    
    // Zwiekszenie zrecznosci
    pub fn add_dex(&mut self, vid: u32, amount: u32) -> bool {
        if let Some(player) = self.stats.get_mut(&vid) {
            player.dex_points = player.dex_points.saturating_add(amount);
            true
        } else {
            false
        }
    }

    // Zwiekszenie inteligencji
    pub fn add_int(&mut self, vid: u32, amount: u32) -> bool {
        if let Some(player) = self.stats.get_mut(&vid) {
            player.int_points = player.int_points.saturating_add(amount);
            true
        } else {
            false
        }
    }

    // Zwiekszenie kondycji
    pub fn add_con(&mut self, vid: u32, amount: u32) -> bool {
        if let Some(player) = self.stats.get_mut(&vid) {
            player.con_points = player.con_points.saturating_add(amount);
            true
        } else {
            false
        }
    }
}
