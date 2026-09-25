//! Modul dxt5_decoder dla skrzynki m2-client-image
//! Dekoder kompresji blokowej DXT5/BC3 (interpolowany 8-stopniowy kanal alpha)
//! Dekompresja blokow 4x4 DXT5, 8-stopniowa interpolacja wartosci alpha

/// Glowna struktura implementacyjna
#[derive(Debug, Clone, Default)]
pub struct Dxt5Decoder {
    pub initialized: bool,
}

impl Dxt5Decoder {
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
    fn test_skeleton_dxt5_decoder() {
        let obj = Dxt5Decoder::new();
        assert!(obj.is_ready());
    }
}
