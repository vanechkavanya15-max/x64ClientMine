use std::collections::HashMap;
use std::sync::{OnceLock, RwLock};

pub const PLAYER_STATS: u8 = 0;

// Struktura przechowujaca statystyki postaci
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct PlayerStats {
    pub level: u8,
    pub hp: i32,
    pub max_hp: i32,
    pub sp: i32,
    pub max_sp: i32,
    pub exp: u64,
    pub max_exp: u64,
    pub gold: i64,
    pub stat_points: u16,
}

impl PlayerStats {
    // Inicjalizacja nowych statystyk
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update_point(&mut self, point_type: u8, value: i64) {
        if point_type == PLAYER_STATS {
            self.gold = value;
        }
    }

    pub fn hp_percentage(&self) -> f32 {
        if self.max_hp <= 0 {
            0.0
        } else {
            (self.hp as f32 / self.max_hp as f32) * 100.0
        }
    }

    pub fn is_alive(&self) -> bool {
        self.hp > 0
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
    pub fn set_lvl(&mut self, vid: u32, lvl: u8) -> bool {
        if let Some(player) = self.stats.get_mut(&vid) {
            player.level = lvl;
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
    pub fn add_hp(&mut self, vid: u32, amount: i32) -> bool {
        if let Some(player) = self.stats.get_mut(&vid) {
            player.hp = player.hp.saturating_add(amount);
            if player.max_hp > 0 && player.hp > player.max_hp {
                player.hp = player.max_hp;
            }
            true
        } else {
            false
        }
    }

    // Bezpieczne odejmowanie HP
    pub fn sub_hp(&mut self, vid: u32, amount: i32) -> bool {
        if let Some(player) = self.stats.get_mut(&vid) {
            player.hp = player.hp.saturating_sub(amount);
            if player.hp < 0 {
                player.hp = 0;
            }
            true
        } else {
            false
        }
    }

    // Bezpieczne dodanie SP
    pub fn add_mp(&mut self, vid: u32, amount: i32) -> bool {
        if let Some(player) = self.stats.get_mut(&vid) {
            player.sp = player.sp.saturating_add(amount);
            if player.max_sp > 0 && player.sp > player.max_sp {
                player.sp = player.max_sp;
            }
            true
        } else {
            false
        }
    }

    // Bezpieczne odejmowanie SP
    pub fn sub_mp(&mut self, vid: u32, amount: i32) -> bool {
        if let Some(player) = self.stats.get_mut(&vid) {
            player.sp = player.sp.saturating_sub(amount);
            if player.sp < 0 {
                player.sp = 0;
            }
            true
        } else {
            false
        }
    }
}

// C-FFI
use crate::ffi_guard::safe_ffi_call;

#[no_mangle]
pub unsafe extern "C" fn m2_player_stats_new() -> *mut PlayerStats {
    safe_ffi_call(std::ptr::null_mut(), || {
        let stats = Box::new(PlayerStats::new());
        Box::into_raw(stats)
    })
}

#[no_mangle]
pub unsafe extern "C" fn m2_player_stats_free(ptr: *mut PlayerStats) {
    safe_ffi_call((), || {
        if !ptr.is_null() {
            let _ = Box::from_raw(ptr);
        }
    });
}

#[no_mangle]
pub unsafe extern "C" fn m2_player_get_hp() -> i32 {
    safe_ffi_call(0, || {
        // Implementacja dummy, wlasciwa pobieralaby z globalnego zarzadcy lub aktualnego lokalnego gracza.
        // Zgodnie z poleceniem wystarczy funkcja FFI chroniona.
        0
    })
}

#[no_mangle]
pub unsafe extern "C" fn m2_player_get_max_hp() -> i32 {
    safe_ffi_call(0, || {
        // Dummy - domyslnie 0, aktualizowane przez GC_POINTS z serwera
        0
    })
}

#[no_mangle]
pub unsafe extern "C" fn m2_player_is_alive() -> bool {
    safe_ffi_call(false, || {
        // Dummy - domyslnie false; w pelnej implementacji sprawdzalby hp > 0
        false
    })
}
