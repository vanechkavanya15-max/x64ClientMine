//! Modul `input_mouse` — Sterowanie postacia myszka (LPM chodzenie/atak, PPM kamera)
//! Wzorzec referencyjny C++: `source/UserInterface/PythonPlayerInputMouse.cpp`
//! Opis: Odwzorowanie __OnClickActor, __OnClickGround, __OnPressActor: klikniecie w ziemie wyznacza cel marszu, klikniecie na potwora uruchamia podbiegniecie i atak.

#[derive(Debug, Clone, Default)]
pub struct InputMouse {
    pub initialized: bool,
}

impl InputMouse {
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
    fn test_skeleton_input_mouse() {
        let instance = InputMouse::new();
        assert!(instance.is_ready());
    }
}
