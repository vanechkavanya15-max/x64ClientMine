//! Modul subimage_extractor dla skrzynki m2-client-image
//! Parser deskryptorow atlasow pod-obrazow Metin2 (.sub)
//! Odczyt plikow .sub, sciezka do atlasu bazowego, prostokat wycinka UV (left, top, right, bottom)

/// Glowna struktura implementacyjna
#[derive(Debug, Clone, Default)]
pub struct SubimageExtractor {
    pub initialized: bool,
}

impl SubimageExtractor {
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
    fn test_skeleton_subimage_extractor() {
        let obj = SubimageExtractor::new();
        assert!(obj.is_ready());
    }
}
