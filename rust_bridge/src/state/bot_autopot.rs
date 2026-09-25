use std::collections::HashMap;

// Konfiguracja dla pojedynczego rodzaju mikstury (HP lub MP).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PotionConfig {
    pub enabled: bool,
    pub threshold_percent: u8,
}

// Konfiguracja automatycznego uzywania mikstur dla danego bytu (gracza, zwierzaka itp.).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AutopotConfig {
    pub hp: PotionConfig,
    pub mp: PotionConfig,
}

// Menedzer stanu automatycznego uzywania mikstur.
// Wykorzystuje strukture HashMap dla bezpiecznego przechowywania danych z kluczem VID.
pub struct AutopotManager {
    configs: HashMap<u32, AutopotConfig>,
}

impl AutopotManager {
    // Tworzy nowa, pusta instancje menedzera.
    pub fn new() -> Self {
        Self {
            configs: HashMap::new(),
        }
    }

    // Ustawia konfiguracje mikstur HP dla zadanego VID.
    // Zapewnia, ze prog procentowy nie przekroczy 100%.
    pub fn set_hp_config(&mut self, vid: u32, enabled: bool, mut threshold_percent: u8) {
        if threshold_percent > 100 {
            threshold_percent = 100;
        }
        let config = self.configs.entry(vid).or_default();
        config.hp.enabled = enabled;
        config.hp.threshold_percent = threshold_percent;
    }

    // Ustawia konfiguracje mikstur MP dla zadanego VID.
    // Zapewnia, ze prog procentowy nie przekroczy 100%.
    pub fn set_mp_config(&mut self, vid: u32, enabled: bool, mut threshold_percent: u8) {
        if threshold_percent > 100 {
            threshold_percent = 100;
        }
        let config = self.configs.entry(vid).or_default();
        config.mp.enabled = enabled;
        config.mp.threshold_percent = threshold_percent;
    }

    // Pobiera cala konfiguracje dla zadanego VID.
    // Odporne na bledne lub nieistniejace VID (zwraca po prostu None).
    pub fn get_config(&self, vid: u32) -> Option<&AutopotConfig> {
        self.configs.get(&vid)
    }

    // Usuwa konfiguracje dla zadanego VID z pamieci.
    pub fn remove_config(&mut self, vid: u32) {
        self.configs.remove(&vid);
    }

    // Sprawdza, czy mikstura HP powinna zostac uzyta.
    // Metoda jest bezpieczna przed bledami dzielenia przez zero.
    pub fn should_use_hp(&self, vid: u32, current_hp: u32, max_hp: u32) -> bool {
        if max_hp == 0 {
            return false;
        }

        if let Some(config) = self.configs.get(&vid) {
            if !config.hp.enabled {
                return false;
            }

            // Obliczenie aktualnego procentu HP za pomoca zmiennej u64,
            // by uniknac ewentualnego przepelnienia (choc dla u32 * 100 jest bezpieczne na u64).
            let mut current_percent = (current_hp as u64 * 100) / (max_hp as u64);
            if current_percent > 100 {
                current_percent = 100;
            }
            return (current_percent as u8) <= config.hp.threshold_percent;
        }

        false
    }

    // Sprawdza, czy mikstura MP powinna zostac uzyta.
    // Metoda jest bezpieczna przed bledami dzielenia przez zero.
    pub fn should_use_mp(&self, vid: u32, current_mp: u32, max_mp: u32) -> bool {
        if max_mp == 0 {
            return false;
        }

        if let Some(config) = self.configs.get(&vid) {
            if !config.mp.enabled {
                return false;
            }

            // Obliczenie aktualnego procentu MP za pomoca zmiennej u64.
            let mut current_percent = (current_mp as u64 * 100) / (max_mp as u64);
            if current_percent > 100 {
                current_percent = 100;
            }
            return (current_percent as u8) <= config.mp.threshold_percent;
        }

        false
    }
}
