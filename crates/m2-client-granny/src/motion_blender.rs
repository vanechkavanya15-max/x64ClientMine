//! Modul motion_blender dla skrzynki m2-client-granny
//! Plynne blendowanie i crossfading pomiedzy stanami animacji
//! Plynne przejscia z biegu do ataku bez szarpniec, czas przenikania 0.1s-0.2s

/// Glowna struktura implementacyjna
#[derive(Debug, Clone, Default)]
pub struct MotionBlender {
    pub initialized: bool,
}

impl MotionBlender {
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
    fn test_skeleton_motion_blender() {
        let obj = MotionBlender::new();
        assert!(obj.is_ready());
    }
}
