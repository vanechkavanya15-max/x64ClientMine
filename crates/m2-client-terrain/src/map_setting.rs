//! Modul map_setting dla skrzynki m2-client-terrain
//! Parser konfiguracji mapy Setting.txt
//! Odczyt wymiarow siatki, skali wysokosci, sciezki do textureset i otoczenia

/// Glowna struktura implementacyjna
#[derive(Debug, Clone, Default)]
pub struct MapSetting {
    pub initialized: bool,
}

impl MapSetting {
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
    fn test_skeleton_map_setting() {
        let obj = MapSetting::new();
        assert!(obj.is_ready());
    }
}
