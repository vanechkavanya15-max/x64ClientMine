//! Modul water_map dla skrzynki m2-client-terrain
//! Parser plaszczyzny wody .wat
//! Odczyt siatki obecnosci wody i wysokosci tafli wodnej

/// Glowna struktura implementacyjna
#[derive(Debug, Clone, Default)]
pub struct WaterMap {
    pub initialized: bool,
}

impl WaterMap {
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
    fn test_skeleton_water_map() {
        let obj = WaterMap::new();
        assert!(obj.is_ready());
    }
}
