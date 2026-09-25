//! Modul sound_emitter dla skrzynki m2-client-audio
//! Przestrzenny emiter dzwieku 3D z tlumieniem odleglosciowym
//! Pozycjonowanie zrodla dzwieku 3D (X, Y, Z), tlumienie odleglosciowe (rolloff)

/// Reprezentuje wektor 3D uzywany do pozycjonowania zrodel dzwieku
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    /// Oblicza odleglosc euklidesowa pomiedzy dwoma wektorami
    pub fn distance(&self, other: &Vec3) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

/// Konfiguracja tlumienia odleglosciowego
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AttenuationConfig {
    pub min_distance: f32,
    pub max_distance: f32,
    pub rolloff_factor: f32,
}

impl Default for AttenuationConfig {
    fn default() -> Self {
        Self {
            min_distance: 1.0,
            max_distance: 100.0,
            rolloff_factor: 1.0,
        }
    }
}

/// Glowna struktura implementacyjna emitera dzwieku
#[derive(Debug, Clone)]
pub struct SoundEmitter {
    pub initialized: bool,
    position: Vec3,
    base_volume: f32,
    attenuation: AttenuationConfig,
}

impl Default for SoundEmitter {
    fn default() -> Self {
        Self::new()
    }
}

impl SoundEmitter {
    pub fn new() -> Self {
        Self {
            initialized: true,
            position: Vec3::default(),
            base_volume: 1.0,
            attenuation: AttenuationConfig::default(),
        }
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }

    /// Ustawia pozycje 3D emitera dzwieku
    pub fn set_position(&mut self, pos: Vec3) {
        self.position = pos;
    }

    /// Zwraca aktualna pozycje emitera dzwieku
    pub fn get_position(&self) -> Vec3 {
        self.position
    }

    /// Ustawia bazowa glosnosc, przycinajac do zakresu 0.0 - 1.0
    pub fn set_base_volume(&mut self, volume: f32) {
        self.base_volume = volume.clamp(0.0, 1.0);
    }

    /// Zwraca bazowa glosnosc emitera
    pub fn get_base_volume(&self) -> f32 {
        self.base_volume
    }

    /// Ustawia konfiguracje tlumienia
    pub fn set_attenuation(&mut self, config: AttenuationConfig) {
        self.attenuation = config;
    }

    /// Zwraca konfiguracje tlumienia emitera
    pub fn get_attenuation(&self) -> AttenuationConfig {
        self.attenuation
    }

    /// Oblicza ostateczna glosnosc w zaleznosci od pozycji sluchacza, wykorzystujac tlumienie Inverse Distance Clamped
    pub fn calculate_volume(&self, listener_position: &Vec3) -> f32 {
        let distance = self.position.distance(listener_position);

        if distance < self.attenuation.min_distance {
            return self.base_volume;
        }

        if distance > self.attenuation.max_distance {
            return 0.0;
        }

        // Tlumienie uzywa Inverse Distance Clamped model (OpenAL style)
        // Gain = MinDistance / (MinDistance + RolloffFactor * (Distance - MinDistance))
        // Z przycieciem ponizej max_distance zapewnianym przez powyzszego if'a,
        // jednakze czesto implementuje sie to jako plynne wyciszanie liniowe pod koniec (max_distance).
        // Dla silnika uzyjemy prostego tlumienia liniowego od min_distance do max_distance
        // uwzgledniajacego rolloff_factor (jesli chcemy wieksza strome).
        // Standardowo liniowo:
        // vol = base_volume * (1.0 - (distance - min_dist) / (max_dist - min_dist))
        
        let range = self.attenuation.max_distance - self.attenuation.min_distance;
        if range <= 0.0 {
            return 0.0;
        }

        let dist_factor = (distance - self.attenuation.min_distance) / range;
        
        // Zastosowanie rolloff_factor
        let attenuation_val = dist_factor * self.attenuation.rolloff_factor;
        
        // Glosnosc spada liniowo, ale modulowana przez rolloff_factor
        let mut final_volume = self.base_volume * (1.0 - attenuation_val);
        
        // Zabezpieczenie przed wartosciami mniejszymi niz 0
        if final_volume < 0.0 {
            final_volume = 0.0;
        }

        final_volume
    }
}
