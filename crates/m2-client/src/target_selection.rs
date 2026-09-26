// Menedzer Wyboru Celu i Okna Informacji o Celu
// Obowiazuje BEZWZGLEDNY ZAKAZ chodzenia na skroty - pelna, rzeczywista logika biznesowa.

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct TargetManager {
    pub selected_target_vid: Option<u32>,
    pub target_hp_percentage: u8,
}

impl TargetManager {
    /// Tworzy nowa, pusta instancje menedzera celu.
    pub fn new() -> Self {
        Self::default()
    }

    /// Ustawia nowy cel gracza i natychmiast kalkuluje jego punkty zycia.
    pub fn select_target(&mut self, vid: u32, hp: i32, max_hp: i32) {
        self.selected_target_vid = Some(vid);
        
        // Zabezpieczenie przed podaniem zlych danych bazowych
        if max_hp <= 0 {
            self.target_hp_percentage = 0;
            return;
        }
        
        // Kalkulacja HP. Uzywamy update_target_hp, aby nie duplikowac kodu,
        // jednak najpierw sprawdzamy hp <= 0. Jesli hp <= 0, cel jest martwy od razu.
        self.update_target_hp(hp, max_hp);
    }

    /// Aktualizuje pasek HP zaznaczonego celu (0..100%).
    /// Automatycznie odznacza cel, jesli jego HP spadnie do 0 lub ponizej (smierc).
    pub fn update_target_hp(&mut self, current_hp: i32, max_hp: i32) {
        if self.selected_target_vid.is_none() {
            return;
        }

        if current_hp <= 0 {
            self.clear_target();
            return;
        }

        if max_hp <= 0 {
            self.target_hp_percentage = 0;
            return;
        }

        // Zabezpieczenie przed wyjsciem poza skale 0-100%
        let clamped_hp = current_hp.min(max_hp);
        
        // Precyzyjna kalkulacja procentowa zapobiegajaca bledom dzielenia calkowitoliczbowego
        let percentage = (clamped_hp as i64 * 100) / (max_hp as i64);
        
        self.target_hp_percentage = percentage.clamp(0, 100) as u8;
    }

    /// Czysto zamyka pasek celu w GUI i odznacza go z pamieci.
    pub fn clear_target(&mut self) {
        self.selected_target_vid = None;
        self.target_hp_percentage = 0;
    }

    /// Sprawdza odleglosc do celu. 
    /// Automatycznie odznacza cel, gdy oddali sie poza zasieg widzenia.
    pub fn check_target_distance(&mut self, distance: f32, max_distance: f32) {
        if self.selected_target_vid.is_some() {
            if distance > max_distance {
                self.clear_target();
            }
        }
    }
}
