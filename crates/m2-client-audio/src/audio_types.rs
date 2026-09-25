//! Modul audio_types dla skrzynki m2-client-audio
//! Typy podstawowe audio: SoundCategory, glosnosc, panorama stereo
//! Struktury SoundCategory (UI, EFFECT, BGM, VOICE), AudioParams (volume, pan, pitch)

/// Kategorie dzwiekow w silniku audio
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SoundCategory {
    /// Dzwieki interfejsu uzytkownika
    UI,
    /// Efekty dzwiekowe srodowiska i postaci
    EFFECT,
    /// Muzyka w tle
    BGM,
    /// Kwestie dialogowe i glosy postaci
    VOICE,
}

impl Default for SoundCategory {
    fn default() -> Self {
        SoundCategory::EFFECT
    }
}

/// Parametry odtwarzania dzwieku
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AudioParams {
    /// Glosnosc dzwieku (zazwyczaj 0.0 - 1.0)
    pub volume: f32,
    /// Panorama stereo (zazwyczaj -1.0 lewo, 1.0 prawo)
    pub pan: f32,
    /// Wysokosc dzwieku (zazwyczaj 1.0 to domyslna wysokosc)
    pub pitch: f32,
}

impl Default for AudioParams {
    fn default() -> Self {
        Self {
            volume: 1.0,
            pan: 0.0,
            pitch: 1.0,
        }
    }
}

impl AudioParams {
    /// Tworzy nowe parametry z domyslnymi wartosciami
    pub fn new() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sound_category_default() {
        assert_eq!(SoundCategory::default(), SoundCategory::EFFECT);
    }

    #[test]
    fn test_audio_params_default() {
        let params = AudioParams::default();
        assert_eq!(params.volume, 1.0);
        assert_eq!(params.pan, 0.0);
        assert_eq!(params.pitch, 1.0);
    }
}
