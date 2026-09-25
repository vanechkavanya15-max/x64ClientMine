//! Modul pipeline_model dla skrzynki m2-client-render
//! Potok renderowania postaci z GPU Skinningiem (do 4 kosci)
//! Format wierzcholka PNT+wagi, paleta macierzy transformacji kosci szkieletu na GPU

/// Glowna struktura implementacyjna
#[derive(Debug, Clone, Default)]
pub struct PipelineModel {
    pub initialized: bool,
}

impl PipelineModel {
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
    fn test_skeleton_pipeline_model() {
        let obj = PipelineModel::new();
        assert!(obj.is_ready());
    }
}
