//! Modul pipeline_effect dla skrzynki m2-client-render
//! Potok renderowania czasteczek (Particles / Billboards)
//! Czworokaty zwrocone w strone kamery (billboards), tryb mieszania Additive i AlphaBlend

/// Glowna struktura implementacyjna
#[derive(Debug, Clone, Default)]
pub struct PipelineEffect {
    pub initialized: bool,
}

impl PipelineEffect {
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
    fn test_skeleton_pipeline_effect() {
        let obj = PipelineEffect::new();
        assert!(obj.is_ready());
    }
}
