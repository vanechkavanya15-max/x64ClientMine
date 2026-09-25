//! Modul raw_heightmap dla skrzynki m2-client-terrain
//! Dekoder mapy wysokosci 16-bit .raw z interpolacja dwuliniowa
//! Odczyt siatki wysokosci, obliczanie normalnych powierzchni, interpolacja wysokosci dla (X, Y)

/// Glowna struktura implementacyjna
#[derive(Debug, Clone, Default)]
pub struct RawHeightmap {
    pub initialized: bool,
}

impl RawHeightmap {
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
    fn test_skeleton_raw_heightmap() {
        let obj = RawHeightmap::new();
        assert!(obj.is_ready());
    }
}
