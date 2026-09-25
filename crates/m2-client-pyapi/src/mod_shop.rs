//! Modul mod_shop dla skrzynki m2-client-pyapi
//! Modul Pythona shop: sklepy NPC i sklepy graczy
//! Metody shop.IsOpen, shop.GetItemVnum, shop.GetItemCount, shop.GetItemPrice

/// Glowna struktura implementacyjna
#[derive(Debug, Clone, Default)]
pub struct ModShop {
    pub initialized: bool,
}

impl ModShop {
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
    fn test_skeleton_mod_shop() {
        let obj = ModShop::new();
        assert!(obj.is_ready());
    }
}
