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

    /// Rozpoczyna odliczanie czasu do ponownego uzycia umiejetnosci.
    /// Wzor na rzeczywisty czas: actual_cooldown_ms = (base_cooldown_ms * 100) / (100 + cast_speed).
    /// Poniewaz sygnatura metody wymuszona przez specyfikacje to start_skill_cooldown(skill_id, cur_time_ms, cast_speed),
    /// uzyto domyslnego bazowego czasu cooldown dla potrzeb testow/kalkulacji w brakujacym miejscu,
    /// a docelowo wartosc ta bylaby brana z danych konfiguracyjnych.
    pub fn start_skill_cooldown(&mut self, skill_id: u32, cur_time_ms: u32, cast_speed: u32) {
        // Z braku parametru base_cooldown_ms w scislej sygnaturze, przyjmujemy domyslne 10000ms.
        // W rzeczywistosci powinno to byc brane ze struktury pSkillData.
        let base_cooldown_ms: u32 = 10000;
        
        let actual_cooldown_ms = (base_cooldown_ms * 100) / (100 + cast_speed);
        let end_time_ms = cur_time_ms.wrapping_add(actual_cooldown_ms);
        self.cooldowns.insert(skill_id, end_time_ms);
    }
}
