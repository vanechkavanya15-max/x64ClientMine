//! Modul render_scene dla skrzynki m2-client-render
//! Glowny orkiestrator kolejki rysowania klatki sceny
//! Sortowanie obiektow (nieprzezroczyste przod-tyl, przezroczyste tyl-przod), statystyki klatki

/// Glowna struktura implementacyjna
#[derive(Debug, Clone, Default)]
pub struct RenderScene {
    pub initialized: bool,
}

impl RenderScene {
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
    fn test_skeleton_render_scene() {
        let obj = RenderScene::new();
        assert!(obj.is_ready());
    }
}
