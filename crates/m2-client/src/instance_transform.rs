//! Modul `instance_transform` — Transformacje przestrzenne i orientacja aktora
//! Wzorzec referencyjny C++: `source/UserInterface/InstanceBaseTransform.cpp`
//! Opis: Odwzorowanie NEW_GetPixelPosition, NEW_SetPixelPosition, LookAt, obliczanie odleglosci 2D/3D pomiedzy dwoma postaciami.

#[derive(Debug, Clone, Default)]
pub struct InstanceTransform {
    pub initialized: bool,
}

impl InstanceTransform {
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
    fn test_skeleton_instance_transform() {
        let instance = InstanceTransform::new();
        assert!(instance.is_ready());
    }
}
