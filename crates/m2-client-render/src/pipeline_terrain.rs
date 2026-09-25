//! Modul pipeline_terrain dla skrzynki m2-client-render
//! Potok renderowania terenu ze splattingiem 4 tekstur
//! Format wierzcholka terenu, bufor indeksow, mieszanie tekstur wg wag alfa

/// Glowna struktura implementacyjna
#[derive(Debug, Clone, Default)]
pub struct PipelineTerrain {
    pub initialized: bool,
}

impl PipelineTerrain {
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
    fn test_skeleton_pipeline_terrain() {
        let obj = PipelineTerrain::new();
        assert!(obj.is_ready());
    }
}
