//! Modul wav_decoder dla skrzynki m2-client-audio
//! Dekoder nieskompresowanych plikow WAV PCM (8/16-bit)
//! Parser naglowkow RIFF/WAVE, format PCM, odczyt probek audio mono/stereo

/// Glowna struktura implementacyjna
#[derive(Debug, Clone, Default)]
pub struct WavDecoder {
    pub initialized: bool,
}

impl WavDecoder {
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
    fn test_skeleton_wav_decoder() {
        let obj = WavDecoder::new();
        assert!(obj.is_ready());
    }
}
