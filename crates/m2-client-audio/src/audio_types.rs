//! Modul audio_types dla skrzynki m2-client-audio
//! Typy podstawowe audio: SoundCategory, glosnosc, panorama stereo
//! Struktury SoundCategory (UI, EFFECT, BGM, VOICE), AudioParams (volume, pan, pitch)

/// Glowna struktura implementacyjna
#[derive(Debug, Clone, Default)]
pub struct AudioTypes {
    pub initialized: bool,
}

impl AudioTypes {
    pub fn new() -> Self {
        Self { initialized: true }
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skeleton_audio_types() {
        let obj = AudioTypes::new();
        assert!(obj.is_ready());
    }
}
