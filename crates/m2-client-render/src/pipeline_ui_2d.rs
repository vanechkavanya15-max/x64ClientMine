//! Modul pipeline_ui_2d dla skrzynki m2-client-render
//! Potok wsadowego renderowania interfejsu 2D (UI Batcher)
//! Rzutowanie ortogonalne 2D, wsadowe rysowanie kafelkow tekstur i ikon okienkowych

/// Glowna struktura implementacyjna
#[derive(Debug, Clone, Default)]
pub struct PipelineUi2d {
    pub initialized: bool,
}

impl PipelineUi2d {
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
    fn test_skeleton_pipeline_ui_2d() {
        let obj = PipelineUi2d::new();
        assert!(obj.is_ready());
    }
}
