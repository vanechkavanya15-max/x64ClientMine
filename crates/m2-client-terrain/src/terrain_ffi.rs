//! Modul terrain_ffi dla skrzynki m2-client-terrain
//! Mostek C-FFI dla operacji na terenie
//! Eksporty C-FFI m2_terrain_load, m2_terrain_get_height, m2_terrain_is_blocked, m2_terrain_free

/// Glowna struktura implementacyjna
#[derive(Debug, Clone, Default)]
pub struct TerrainFfi {
    pub initialized: bool,
}

impl TerrainFfi {
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
    fn test_skeleton_terrain_ffi() {
        let obj = TerrainFfi::new();
        assert!(obj.is_ready());
    }
}
