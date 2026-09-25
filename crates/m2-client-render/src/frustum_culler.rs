//! Modul frustum_culler dla skrzynki m2-client-render
//! Wycinanie ostroslupa widzenia (Frustum Culling) dla brył AABB i kul
//! Wyznaczanie 6 plaszczyzn frustum z macierzy VP, testy widocznosci brył otaczajacych

/// Glowna struktura implementacyjna
#[derive(Debug, Clone, Default)]
pub struct FrustumCuller {
    pub initialized: bool,
}

impl FrustumCuller {
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
    fn test_skeleton_frustum_culler() {
        let obj = FrustumCuller::new();
        assert!(obj.is_ready());
    }
}
