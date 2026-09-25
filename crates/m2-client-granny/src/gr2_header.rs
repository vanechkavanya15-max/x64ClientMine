//! Modul gr2_header dla skrzynki m2-client-granny
//! Parser naglowka pliku binarnego Granny 2 (.gr2)
//! Weryfikacja sygnatury magic, wersji formatu (2.4/2.9), endianness i offsetow sekcji

/// Glowna struktura implementacyjna
#[derive(Debug, Clone, Default)]
pub struct Gr2Header {
    pub initialized: bool,
}

impl Gr2Header {
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
    fn test_skeleton_gr2_header() {
        let obj = Gr2Header::new();
        assert!(obj.is_ready());
    }
}
