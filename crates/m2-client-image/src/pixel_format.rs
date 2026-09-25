//! Modul pixel_format dla skrzynki m2-client-image
//! Formaty pikseli i konwersje przestrzeni barwnych
//! PixelFormat (RGBA8, BGRA8, RGB565, ARGB1555, DXT1, DXT3, DXT5) oraz konwersje

/// Glowna struktura implementacyjna
#[derive(Debug, Clone, Default)]
pub struct PixelFormat {
    pub initialized: bool,
}

impl PixelFormat {
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
    fn test_skeleton_pixel_format() {
        let obj = PixelFormat::new();
        assert!(obj.is_ready());
    }
}
