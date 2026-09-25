//! Modul font_atlas dla skrzynki m2-client-render
//! Rasteryzator czcionek i atlas glifow tekstowych
//! Pomiary szerokosci tekstu, uklad linii napisow nad postaciami i w dymkach czatu

/// Glowna struktura implementacyjna
#[derive(Debug, Clone, Default)]
pub struct FontAtlas {
    pub initialized: bool,
}

impl FontAtlas {
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
    fn test_skeleton_font_atlas() {
        let obj = FontAtlas::new();
        assert!(obj.is_ready());
    }
}
