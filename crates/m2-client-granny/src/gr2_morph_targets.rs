//! Modul gr2_morph_targets dla skrzynki m2-client-granny
//! Obsluga celow ksztaltu (Morph Targets) i animacji mimiki
//! Przesuniecia wierzcholkow delta positions, wagi blendshape

/// Glowna struktura implementacyjna
#[derive(Debug, Clone, Default)]
pub struct Gr2MorphTargets {
    pub initialized: bool,
}

impl Gr2MorphTargets {
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
    fn test_skeleton_gr2_morph_targets() {
        let obj = Gr2MorphTargets::new();
        assert!(obj.is_ready());
    }
}
