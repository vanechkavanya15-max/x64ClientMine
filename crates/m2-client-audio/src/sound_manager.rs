//! Modul sound_manager dla skrzynki m2-client-audio
//! Zarzadca efektow dzwiekowych 2D/3D z limitami jednoczesnych probek
//! Odtwarzanie dzwiekow uderzen, krokow, magii, pule odtwarzaczy i cooldowny

use std::collections::HashMap;
use std::time::{Duration, Instant};
use crate::audio_types::{AudioParams, SoundCategory};

/// Identyfikator odtwarzanego dzwieku
pub type SoundInstanceId = u64;

/// Stan pojedynczej instancji dzwieku
#[derive(Debug, Clone)]
pub struct ActiveSoundInstance {
    pub id: SoundInstanceId,
    pub sound_name: String,
    pub category: SoundCategory,
    pub params: AudioParams,
    pub start_time: Instant,
    pub duration: Duration,
    pub is_looping: bool,
    pub is_paused: bool,
}

/// Menedzer efektow dzwiekowych z limitami kanalow i cooldownami
#[derive(Debug)]
pub struct SoundManager {
    pub initialized: bool,
    /// Maksymalna dopuszczalna liczba jednoczesnie odtwarzanych dzwiekow
    pub max_concurrent_sounds: usize,
    /// Glosnosc glowna (0.0 do 1.0)
    pub master_volume: f32,
    /// Glosnosci kategorii dzwiekowych
    pub category_volumes: HashMap<SoundCategory, f32>,
    /// Aktywnie odtwarzane instancje
    active_instances: HashMap<SoundInstanceId, ActiveSoundInstance>,
    /// Ostatnie czasy odtworzenia poszczegolnych dzwiekow (dla cooldownow)
    last_played: HashMap<String, Instant>,
    /// Minimalny czas pomiedzy powtorzeniem tego samego dzwieku
    cooldown: Duration,
    /// Licznik generowanych ID
    next_id: SoundInstanceId,
}

impl Default for SoundManager {
    fn default() -> Self {
        Self::new()
    }
}

impl SoundManager {
    /// Tworzy nowego zarzadce dzwiekow z domyslnymi limitami
    pub fn new() -> Self {
        let mut category_volumes = HashMap::new();
        category_volumes.insert(SoundCategory::EFFECT, 1.0);
        category_volumes.insert(SoundCategory::UI, 1.0);
        category_volumes.insert(SoundCategory::VOICE, 1.0);
        category_volumes.insert(SoundCategory::BGM, 0.8);

        Self {
            initialized: true,
            max_concurrent_sounds: 32,
            master_volume: 1.0,
            category_volumes,
            active_instances: HashMap::new(),
            last_played: HashMap::new(),
            cooldown: Duration::from_millis(50),
            next_id: 1,
        }
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }

    /// Ustawia minimalny cooldown pomiedzy odtworzeniami tego samego dzwieku
    pub fn set_cooldown(&mut self, duration: Duration) {
        self.cooldown = duration;
    }

    /// Ustawia globalna glosnosc dla kategorii
    pub fn set_category_volume(&mut self, category: SoundCategory, volume: f32) {
        self.category_volumes.insert(category, volume.clamp(0.0, 1.0));
    }

    /// Pobiera efektywna glosnosc z uwzglednieniem glosnosci glownej
    pub fn get_effective_volume(&self, category: SoundCategory, local_volume: f32) -> f32 {
        let cat_vol = self.category_volumes.get(&category).copied().unwrap_or(1.0);
        (self.master_volume * cat_vol * local_volume).clamp(0.0, 1.0)
    }

    /// Odtwarza dzwiek o podanej nazwie i kategorii
    pub fn play_sound(
        &mut self,
        name: &str,
        category: SoundCategory,
        params: AudioParams,
        duration: Duration,
        is_looping: bool,
    ) -> Result<SoundInstanceId, String> {
        let now = Instant::now();

        // 1. Sprawdz cooldown per dzwiek (ochrona przed nakladaniem sie dzwiekow uderzen/krokow)
        if let Some(&last) = self.last_played.get(name) {
            if now.duration_since(last) < self.cooldown {
                return Err(format!("Dzwiek '{}' zablokowany przez cooldown", name));
            }
        }

        // 2. Czyszczenie zakonczonych dzwiekow
        self.cleanup_finished(now);

        // 3. Sprawdz limit jednoczesnych kanalow
        if self.active_instances.len() >= self.max_concurrent_sounds {
            // Usun najstarszy dzwiek o najnizszym priorytecie
            if let Some(&oldest_id) = self.active_instances.keys().next() {
                self.active_instances.remove(&oldest_id);
            }
        }

        // 4. Utworz nowa instancje
        let id = self.next_id;
        self.next_id += 1;

        let instance = ActiveSoundInstance {
            id,
            sound_name: name.to_string(),
            category,
            params,
            start_time: now,
            duration,
            is_looping,
            is_paused: false,
        };

        self.active_instances.insert(id, instance);
        self.last_played.insert(name.to_string(), now);

        Ok(id)
    }

    /// Zatrzymuje dzwiek o podanym ID
    pub fn stop_sound(&mut self, id: SoundInstanceId) -> bool {
        self.active_instances.remove(&id).is_some()
    }

    /// Zatrzymuje wszystkie dzwieki z podanej kategorii
    pub fn stop_category(&mut self, category: SoundCategory) {
        self.active_instances.retain(|_, inst| inst.category != category);
    }

    /// Zatrzymuje wszystkie aktywne dzwieki
    pub fn stop_all(&mut self) {
        self.active_instances.clear();
    }

    /// Liczba aktualnie odtwarzanych dzwiekow
    pub fn active_count(&self) -> usize {
        self.active_instances.len()
    }

    /// Czyszczenie zakonczonych dzwiekow nieloopujacych
    fn cleanup_finished(&mut self, now: Instant) {
        self.active_instances.retain(|_, inst| {
            if inst.is_looping || inst.is_paused {
                true
            } else {
                now.duration_since(inst.start_time) < inst.duration
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sound_manager_lifecycle() {
        let mut mgr = SoundManager::new();
        assert!(mgr.is_ready());

        let params = AudioParams::default();
        let dur = Duration::from_millis(100);

        let id = mgr.play_sound("sword_hit.wav", SoundCategory::EFFECT, params.clone(), dur, false).unwrap();
        assert_eq!(mgr.active_count(), 1);

        // Natychmiastowe ponowne odtworzenie powinno zostac odrzucone przez cooldown
        let res = mgr.play_sound("sword_hit.wav", SoundCategory::EFFECT, params.clone(), dur, false);
        assert!(res.is_err());

        // Inny dzwiek powinien przejsc
        let _id2 = mgr.play_sound("ui_click.wav", SoundCategory::UI, params, dur, false).unwrap();
        assert_eq!(mgr.active_count(), 2);

        // Zatrzymanie dzwieku
        assert!(mgr.stop_sound(id));
        assert_eq!(mgr.active_count(), 1);

        mgr.stop_all();
        assert_eq!(mgr.active_count(), 0);
    }

    #[test]
    fn test_effective_volume() {
        let mut mgr = SoundManager::new();
        mgr.master_volume = 0.5;
        mgr.set_category_volume(SoundCategory::EFFECT, 0.8);

        let eff = mgr.get_effective_volume(SoundCategory::EFFECT, 1.0);
        assert!((eff - 0.4).abs() < 1e-5);
    }
}
