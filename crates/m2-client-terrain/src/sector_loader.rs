//! Modul sector_loader dla skrzynki m2-client-terrain
//! Menedzer asynchronicznego ladowania sektorow mapy 3x3
//! Streaming sektorow terenu wokol pozycji gracza, doczytywanie w tle

/// Glowna struktura implementacyjna
#[derive(Debug, Clone, Default)]
pub struct SectorLoader {
    pub initialized: bool,
}

impl SectorLoader {
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
    fn test_skeleton_sector_loader() {
        let obj = SectorLoader::new();
        assert!(obj.is_ready());
    }
}
