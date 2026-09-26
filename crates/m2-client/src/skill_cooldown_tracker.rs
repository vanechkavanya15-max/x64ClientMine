use std::collections::HashMap;

/// Modul obslugujacy odliczanie czasu do ponownego uzycia umiejetnosci.
pub struct SkillCooldownTracker {
    cooldowns: HashMap<u32, u32>,
}

impl SkillCooldownTracker {
    /// Tworzy nowa instancje zarzadcy odliczania.
    pub fn new() -> Self {
        Self {
            cooldowns: HashMap::new(),
        }
    }

    /// Sprawdza czy umiejetnosc jest gotowa do uzycia uwzgledniajac przepelnienie licznika (tick wrap-around).
    pub fn is_skill_ready(&self, skill_id: u32, cur_time_ms: u32) -> bool {
        if let Some(&end_time_ms) = self.cooldowns.get(&skill_id) {
            // Obliczamy roznice z uwzglednieniem bezpiecznego zawijania (wrapping subtraction)
            let diff = cur_time_ms.wrapping_sub(end_time_ms);
            
            // Jezeli diff jest bardzo duze, oznacza to, ze cur_time_ms jest mniejsze niz end_time_ms
            // z powodu arytmetyki modulowej, a tym samym czas jeszcze nie minal.
            // Przyjmujemy bezpieczny prog polowy zakresu u32
            if diff < (u32::MAX / 2) {
                return true;
            } else {
                return false;
            }
        }
        true
    }

    /// Rozpoczyna odliczanie czasu ze sprecyzowanym bazowym czasem cooldownu (z pSkillData)
    /// i bonusem szybkosci zaklecia (cast_speed: 0 = brak bonusu, 20 = +20% szybkosci zaklecia).
    /// Zgodnie z C++ PythonPlayerSkill.cpp: iSpd = 10000 / (100 + bonus_spd).
    pub fn start_skill_cooldown_with_base(&mut self, skill_id: u32, base_cooldown_ms: u32, cur_time_ms: u32, cast_speed_bonus: u32) {
        let actual_cooldown_ms = (base_cooldown_ms as u64 * 100 / (100 + cast_speed_bonus as u64)) as u32;
        let end_time_ms = cur_time_ms.wrapping_add(actual_cooldown_ms);
        self.cooldowns.insert(skill_id, end_time_ms);
    }

    /// Rozpoczyna odliczanie czasu do ponownego uzycia umiejetnosci z domyslnym bazowym czasem 10000ms.
    pub fn start_skill_cooldown(&mut self, skill_id: u32, cur_time_ms: u32, cast_speed_bonus: u32) {
        self.start_skill_cooldown_with_base(skill_id, 10000, cur_time_ms, cast_speed_bonus);
    }
}

static SKILL_COOLDOWN_TRACKER: std::sync::OnceLock<std::sync::Mutex<SkillCooldownTracker>> = std::sync::OnceLock::new();

fn get_tracker() -> std::sync::MutexGuard<'static, SkillCooldownTracker> {
    SKILL_COOLDOWN_TRACKER
        .get_or_init(|| std::sync::Mutex::new(SkillCooldownTracker::new()))
        .lock()
        .unwrap()
}

#[no_mangle]
pub extern "C" fn m2_skill_start_cd(skill_id: u32, base_cd_ms: u32, cur_time_ms: u32, cast_speed: u32) -> bool {
    let mut tracker = get_tracker();
    tracker.start_skill_cooldown_with_base(skill_id, base_cd_ms, cur_time_ms, cast_speed);
    true
}

#[no_mangle]
pub extern "C" fn m2_skill_is_ready(skill_id: u32, cur_time_ms: u32) -> bool {
    let tracker = get_tracker();
    tracker.is_skill_ready(skill_id, cur_time_ms)
}
