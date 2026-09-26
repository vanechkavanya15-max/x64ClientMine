use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

/// Typy umiejetnosci zgodne z C++ ESkillType
pub const SKILL_TYPE_NONE: u8 = 0;
pub const SKILL_TYPE_ACTIVE: u8 = 1;
pub const SKILL_TYPE_SUPPORT: u8 = 2;
pub const SKILL_TYPE_GUILD: u8 = 3;
pub const SKILL_TYPE_HORSE: u8 = 4;
pub const SKILL_TYPE_MAX_NUM: u8 = 5;

/// Struktura przechowujaca dane rejestru umiejetnosci (SkillData)
#[derive(Debug, Clone)]
pub struct SkillData {
    pub skill_id: u32,
    pub skill_type: u8,
    pub max_level: u8,
    pub base_cooldown_ms: [u32; 41],
    pub sp_cost: [u32; 41],
    pub splash_range: [f32; 41],
}

impl Default for SkillData {
    fn default() -> Self {
        Self {
            skill_id: 0,
            skill_type: SKILL_TYPE_NONE,
            max_level: 40,
            base_cooldown_ms: [0; 41],
            sp_cost: [0; 41],
            splash_range: [0.0; 41],
        }
    }
}

/// Aktywny cooldown dla danej umiejetnosci
#[derive(Debug, Clone, Copy)]
pub struct ActiveCooldown {
    pub start_time_ms: u32,
    pub actual_cooldown_ms: u32,
}

/// Główny cache przechowujący dane i cooldowny
pub struct SkillDataCacheV2 {
    skills: HashMap<u32, SkillData>,
    cooldowns: HashMap<u32, ActiveCooldown>,
}

impl SkillDataCacheV2 {
    pub fn new() -> Self {
        Self {
            skills: HashMap::new(),
            cooldowns: HashMap::new(),
        }
    }

    /// Rejestruje dane umiejetnosci
    pub fn register_skill(&mut self, data: SkillData) {
        self.skills.insert(data.skill_id, data);
    }

    /// Zwraca referencje do danych umiejetnosci
    pub fn get_skill(&self, skill_id: u32) -> Option<&SkillData> {
        self.skills.get(&skill_id)
    }

    /// Oblicza czas odnowienia uwzgledniajac bonus_cast_speed
    /// Wzor zgodny z C++: iSpd = 10000 / (100 + bonus_cast_speed)
    /// cooldown = base_cooldown_ms * iSpd / 100
    pub fn calculate_actual_cooldown(&self, skill_id: u32, skill_level: u8, bonus_cast_speed: i32) -> u32 {
        if let Some(skill) = self.get_skill(skill_id) {
            let level = (skill_level as usize).min(40);
            let base_cd = skill.base_cooldown_ms[level];
            
            if base_cd == 0 {
                return 0;
            }

            let div = 100 + bonus_cast_speed;
            let i_spd = if div == 0 {
                // Avoid division by zero, theoretically shouldn't happen but defensive
                10000
            } else {
                10000 / div
            };

            let result = (base_cd as u64 * i_spd as u64) / 100;
            return result as u32;
        }
        0
    }

    /// Rozpoczyna odliczanie czasu odnowienia z zadanym poziomem i statystyką cast_speed
    pub fn start_cooldown(&mut self, skill_id: u32, skill_level: u8, cur_time_ms: u32, bonus_cast_speed: i32) -> u32 {
        let actual_cd = self.calculate_actual_cooldown(skill_id, skill_level, bonus_cast_speed);
        if actual_cd > 0 {
            self.cooldowns.insert(skill_id, ActiveCooldown {
                start_time_ms: cur_time_ms,
                actual_cooldown_ms: actual_cd,
            });
        }
        actual_cd
    }

    /// Zwraca procentowy postęp odnowienia (0.0 .. 1.0) dla shaderów zaciemnienia UI.
    /// Jeśli cooldown się zakończył (lub go nie ma), zwraca 0.0 (gotowy do użycia).
    /// Zmodyfikowana mechanika:
    /// - 1.0 oznacza, że umiejętność jest zablokowana na maksymalny czas (właśnie użyta)
    /// - Maleje do 0.0, kiedy będzie gotowa.
    pub fn get_cooldown_pct(&self, skill_id: u32, cur_time_ms: u32) -> f32 {
        if let Some(cd) = self.cooldowns.get(&skill_id) {
            let elapsed = cur_time_ms.wrapping_sub(cd.start_time_ms);
            
            // Jezeli uplyniety czas przekracza faktyczny cooldown (albo doszlo do duzego underflow), to gotowe
            if elapsed >= cd.actual_cooldown_ms || elapsed > (u32::MAX / 2) {
                return 0.0;
            }

            // Postęp upływu czasu: 0.0 zaraz po uzyciu, 1.0 przed odblokowaniem
            let elapsed_pct = elapsed as f32 / cd.actual_cooldown_ms as f32;

            // Shader na ogol uzywa odwrotnosci - ile procent wypelnic na szaro.
            // Przy uzyciu skilla caly kafelek jest zaciemniony (1.0).
            return 1.0 - elapsed_pct.clamp(0.0, 1.0);
        }
        
        0.0 // Gotowy do użycia
    }
}

// Globalny stan dla eksportu FFI
static SKILL_DATA_CACHE_V2: OnceLock<Mutex<SkillDataCacheV2>> = OnceLock::new();

fn get_cache() -> Option<std::sync::MutexGuard<'static, SkillDataCacheV2>> {
    SKILL_DATA_CACHE_V2
        .get_or_init(|| Mutex::new(SkillDataCacheV2::new()))
        .lock()
        .ok()
}

/// Eksport FFI: Zwraca procentowy postęp odnowienia umiejetnosci (0.0 - gotowy, 1.0 - w pelni zaciemniony).
#[no_mangle]
pub extern "C" fn m2_skill_get_cooldown_pct(skill_id: u32, cur_time_ms: u32) -> f32 {
    if let Some(cache) = get_cache() {
        cache.get_cooldown_pct(skill_id, cur_time_ms)
    } else {
        0.0
    }
}

/// Eksport FFI (pomocniczy): Rozpoczyna odliczanie
#[no_mangle]
pub extern "C" fn m2_skill_start_cooldown_v2(skill_id: u32, skill_level: u8, cur_time_ms: u32, bonus_cast_speed: i32) -> u32 {
    if let Some(mut cache) = get_cache() {
        cache.start_cooldown(skill_id, skill_level, cur_time_ms, bonus_cast_speed)
    } else {
        0
    }
}

/// Eksport FFI (pomocniczy): Pobiera koszt SP
#[no_mangle]
pub extern "C" fn m2_skill_get_sp_cost(skill_id: u32, skill_level: u8) -> u32 {
    if let Some(cache) = get_cache() {
        if let Some(skill) = cache.get_skill(skill_id) {
            let level = (skill_level as usize).min(40);
            return skill.sp_cost[level];
        }
    }
    0
}
