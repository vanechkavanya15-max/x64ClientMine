//! Modul mod_safebox dla skrzynki m2-client-pyapi
//! Modul Pythona safebox: magazyn dozorcy
//! Metody safebox.GetCurrentSafeboxSize, safebox.GetItemID, safebox.GetMoney

/// Glowna struktura implementacyjna
#[derive(Debug, Clone, Default)]
pub struct ModSafebox {
    pub initialized: bool,
}

impl ModSafebox {
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
    fn test_skeleton_mod_safebox() {
        let obj = ModSafebox::new();
        assert!(obj.is_ready());
    }
}
