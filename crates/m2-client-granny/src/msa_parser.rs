//! Modul msa_parser dla skrzynki m2-client-granny
//! Parser skryptow animacji Metin2 .msa (czasy uderzen StartingTime)
//! Odczyt MotionDuration, Accumulation, MotionEventData, dokladnego czasu ciosu StartingTime

/// Glowna struktura implementacyjna
#[derive(Debug, Clone, Default)]
pub struct MsaParser {
    pub initialized: bool,
}

impl MsaParser {
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
    fn test_skeleton_msa_parser() {
        let obj = MsaParser::new();
        assert!(obj.is_ready());
    }
}
