//! Modul ogg_decoder dla skrzynki m2-client-audio
//! Czytnik strumieniowego audio i muzyki tla BGM
//! Strumieniowy czytnik probek, metadane utworu, punkty zapetlenia utworu

/// Glowna struktura implementacyjna
#[derive(Debug, Clone, Default)]
pub struct OggDecoder {
    pub initialized: bool,
}

impl OggDecoder {
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
    fn test_skeleton_ogg_decoder() {
        let obj = OggDecoder::new();
        assert!(obj.is_ready());
    }
}
