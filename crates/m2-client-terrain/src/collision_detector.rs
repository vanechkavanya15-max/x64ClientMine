//! Modul collision_detector dla skrzynki m2-client-terrain
//! Detektor kolizji promienia z terenem i atrybutami
//! Raycast z kamery lub postaci, znajdowanie dokladnej wysokosci Z pod graczem

/// Glowna struktura implementacyjna
#[derive(Debug, Clone, Default)]
pub struct CollisionDetector {
    pub initialized: bool,
}

impl CollisionDetector {
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
    fn test_skeleton_collision_detector() {
        let obj = CollisionDetector::new();
        assert!(obj.is_ready());
    }
}
