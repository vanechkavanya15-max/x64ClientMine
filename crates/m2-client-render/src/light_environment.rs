//! Modul light_environment dla skrzynki m2-client-render
//! Model oswietlenia slonecznego, barwy otoczenia i mgly
//! Wektor slonca, kolor swiatla kierunkowego, kolor ambient, parametry liniowej mgly

/// Glowna struktura implementacyjna
#[derive(Debug, Clone, Default)]
pub struct LightEnvironment {
    pub initialized: bool,
}

impl LightEnvironment {
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
    fn test_skeleton_light_environment() {
        let obj = LightEnvironment::new();
        assert!(obj.is_ready());
    }
}
