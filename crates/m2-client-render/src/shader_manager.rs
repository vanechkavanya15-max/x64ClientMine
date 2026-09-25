//! Modul shader_manager dla skrzynki m2-client-render
//! Katalog i zarzadca shaderow WGSL / HLSL
//! Kompilacja i rejestr programow cieniujacych dla terenu, modeli 3D i interfejsu

/// Glowna struktura implementacyjna
#[derive(Debug, Clone, Default)]
pub struct ShaderManager {
    pub initialized: bool,
}

impl ShaderManager {
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
    fn test_skeleton_shader_manager() {
        let obj = ShaderManager::new();
        assert!(obj.is_ready());
    }
}
