//! Modul gr2_string_table dla skrzynki m2-client-granny
//! Tablica stringow i metadanych definicji typow Granny
//! Odczyt nazw kosci, obiektow, wlasciwosci materialow z tablicy stringow

/// Glowna struktura implementacyjna
#[derive(Debug, Clone, Default)]
pub struct Gr2StringTable {
    pub initialized: bool,
}

impl Gr2StringTable {
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
    fn test_skeleton_gr2_string_table() {
        let obj = Gr2StringTable::new();
        assert!(obj.is_ready());
    }
}
