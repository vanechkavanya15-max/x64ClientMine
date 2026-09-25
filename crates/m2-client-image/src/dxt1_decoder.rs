//! Modul dxt1_decoder dla skrzynki m2-client-image
//! Dekoder kompresji blokowej DXT1/BC1 (nieprzezroczyste/1-bit alpha)
//! Dekompresja blokow 4x4 DXT1, interpolacja kolorow RGB565, 1-bitowa maska alpha

/// Glowna struktura implementacyjna
#[derive(Debug, Clone, Default)]
pub struct Dxt1Decoder {
    pub initialized: bool,
}

impl Dxt1Decoder {
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
    fn test_skeleton_dxt1_decoder() {
        let obj = Dxt1Decoder::new();
        assert!(obj.is_ready());
    }
}
