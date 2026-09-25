//! Modul gpu_context dla skrzynki m2-client-render
//! Kontekst urzadzenia GPU: viewport, bufory koloru i glebi
//! GpuContext, zarzadzanie rozdzielczoscia okna, konfiguracja bufora glebi Z-Buffer

/// Glowna struktura implementacyjna
#[derive(Debug, Clone, Default)]
pub struct GpuContext {
    pub initialized: bool,
}

impl GpuContext {
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
    fn test_skeleton_gpu_context() {
        let obj = GpuContext::new();
        assert!(obj.is_ready());
    }
}
