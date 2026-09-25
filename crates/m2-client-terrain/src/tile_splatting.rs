//! Modul tile_splatting dla skrzynki m2-client-terrain
//! Mieszanie wielowarstwowe tekstur terenu (Alpha Splatting)
//! Odczyt mapy wag alfa dla 4 warstw tekstur na kazdy wierzcholek terenu

/// Glowna struktura implementacyjna
#[derive(Debug, Clone, Default)]
pub struct TileSplatting {
    pub initialized: bool,
}

impl TileSplatting {
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
    fn test_skeleton_tile_splatting() {
        let obj = TileSplatting::new();
        assert!(obj.is_ready());
    }
}
