//! Modul quadtree dla skrzynki m2-client-terrain
//! Drzewo czworkowe (Quadtree) dla culling'u i LOD sektorow
//! Struktura wezlow przestrzennych, prostopadlosciany AABB, testy przeciecia z frustum

/// Glowna struktura implementacyjna
#[derive(Debug, Clone, Default)]
pub struct Quadtree {
    pub initialized: bool,
}

impl Quadtree {
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
    fn test_skeleton_quadtree() {
        let obj = Quadtree::new();
        assert!(obj.is_ready());
    }
}
