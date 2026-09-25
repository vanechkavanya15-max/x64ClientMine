//! Modul render_target dla skrzynki m2-client-render
//! Deskryptor celow renderowania (Render Targets) i antyaliasingu
//! Bufor koloru, bufor glebi/szablonu (Depth-Stencil), konfiguracja MSAA

/// Glowna struktura implementacyjna
#[derive(Debug, Clone, Default)]
pub struct RenderTarget {
    pub initialized: bool,
}

impl RenderTarget {
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
    fn test_skeleton_render_target() {
        let obj = RenderTarget::new();
        assert!(obj.is_ready());
    }
}
