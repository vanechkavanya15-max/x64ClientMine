//! Modul pyapi_ffi dla skrzynki m2-client-pyapi
//! Mostek C-FFI dla integracji interpretera Pythona
//! Eksporty C-FFI m2_pyapi_init, m2_pyapi_dispatch_event, m2_pyapi_shutdown

/// Glowna struktura implementacyjna
#[derive(Debug, Clone, Default)]
pub struct PyapiFfi {
    pub initialized: bool,
}

impl PyapiFfi {
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
    fn test_skeleton_pyapi_ffi() {
        let obj = PyapiFfi::new();
        assert!(obj.is_ready());
    }
}
