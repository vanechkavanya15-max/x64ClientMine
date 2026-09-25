//! Modul dxt3_decoder dla skrzynki m2-client-image
//! Dekoder kompresji blokowej DXT3/BC2 (jawny 4-bitowy kanal alpha)
//! Dekompresja blokow 4x4 DXT3, bezposredni kanal alpha 4 bity na piksel

/// Glowna struktura implementacyjna
#[derive(Debug, Clone, Default)]
pub struct Dxt3Decoder {
    pub initialized: bool,
}

impl Dxt3Decoder {
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
    fn test_skeleton_dxt3_decoder() {
        let obj = Dxt3Decoder::new();
        assert!(obj.is_ready());
    }
}
