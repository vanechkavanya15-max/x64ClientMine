//! Modul image_ffi dla skrzynki m2-client-image
//! Mostek C-FFI dla dekodowania tekstur
//! Eksporty C-FFI m2_image_decode_dxt, m2_image_decode_tga, m2_image_free

/// Glowna struktura implementacyjna
#[derive(Debug, Clone, Default)]
pub struct ImageFfi {
    pub initialized: bool,
}

impl ImageFfi {
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
    fn test_skeleton_image_ffi() {
        let obj = ImageFfi::new();
        assert!(obj.is_ready());
    }
}
