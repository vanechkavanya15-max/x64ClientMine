//! Modul shadow_map dla skrzynki m2-client-terrain
//! Odczyt wypieczonych map cieniowania terenu (.smf / .raw)
//! Natezenie cienia slonecznego, cieniowanie statyczne otoczenia

/// Glowna struktura implementacyjna
#[derive(Debug, Clone, Default)]
pub struct ShadowMap {
    pub initialized: bool,
}

impl ShadowMap {
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
    fn test_skeleton_shadow_map() {
        let obj = ShadowMap::new();
        assert!(obj.is_ready());
    }
}
