//! Modul `instance_movement` — Fizyka ruchu i interpolacja kierunku postaci
//! Wzorzec referencyjny C++: `source/UserInterface/InstanceBaseMovement.cpp`
//! Opis: Odwzorowanie NEW_MoveToDestPixelPositionDirection: obliczanie wektora predkosci na podstawie MoveSpeed, interpolacja pozycji (X, Y) w kazdej klatce oraz katu obrotu.

#[derive(Debug, Clone, Default)]
pub struct InstanceMovement {
    pub initialized: bool,
}

impl InstanceMovement {
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
    fn test_skeleton_instance_movement() {
        let instance = InstanceMovement::new();
        assert!(instance.is_ready());
    }
}
