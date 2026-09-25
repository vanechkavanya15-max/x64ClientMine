//! Modul attr_map dla skrzynki m2-client-terrain
//! Parser mapy atrybutow terenu .atr (kolizje, woda, bezpieczne strefy)
//! Weryfikacja flag BLOCK, WATER, WARP, SAFE_ZONE dla kazdego pola siatki

/// Glowna struktura implementacyjna
#[derive(Debug, Clone, Default)]
pub struct AttrMap {
    pub initialized: bool,
}

impl AttrMap {
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
    fn test_skeleton_attr_map() {
        let obj = AttrMap::new();
        assert!(obj.is_ready());
    }
}
