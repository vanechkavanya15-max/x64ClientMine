//! Modul granny_ffi dla skrzynki m2-client-granny
//! Mostek C-FFI dla ladowania modeli Granny i ewaluacji klatek
//! Eksporty C-FFI m2_granny_load_model, m2_granny_evaluate_pose, m2_granny_free

/// Glowna struktura implementacyjna
#[derive(Debug, Clone, Default)]
pub struct GrannyFfi {
    pub initialized: bool,
}

impl GrannyFfi {
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
    fn test_skeleton_granny_ffi() {
        let obj = GrannyFfi::new();
        assert!(obj.is_ready());
    }
}
