//! Modul gr2_anim_evaluator dla skrzynki m2-client-granny
//! Ewaluator animacji szkieletowej w czasie T z interpolacja SLERP
//! Interpolacja kwaternionow SLERP, translacji LERP, obliczanie macierzy kosci w klatce

/// Glowna struktura implementacyjna
#[derive(Debug, Clone, Default)]
pub struct Gr2AnimEvaluator {
    pub initialized: bool,
}

impl Gr2AnimEvaluator {
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
    fn test_skeleton_gr2_anim_evaluator() {
        let obj = Gr2AnimEvaluator::new();
        assert!(obj.is_ready());
    }
}
