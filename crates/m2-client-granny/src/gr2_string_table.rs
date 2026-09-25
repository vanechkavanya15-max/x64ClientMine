//! Modul gr2_string_table dla skrzynki m2-client-granny
//! Tablica stringow i metadanych definicji typow Granny
//! Odczyt nazw kosci, obiektow, wlasciwosci materialow z tablicy stringow

/// Glowna struktura implementacyjna tablicy stringow
#[derive(Debug, Clone, Default)]
pub struct Gr2StringTable {
    pub data: Vec<u8>,
}

impl Gr2StringTable {
    /// Tworzy nowa, pusta tablice
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    /// Tworzy tablice z gotowego bufora
    pub fn from_buffer(data: Vec<u8>) -> Self {
        Self { data }
    }

    /// Sprawdza czy tablica posiada dane
    pub fn is_ready(&self) -> bool {
        !self.data.is_empty()
    }

    /// Odczytuje C-string (null-terminated) spod podanego offsetu
    pub fn get_string(&self, offset: usize) -> Option<&str> {
        if offset >= self.data.len() {
            return None;
        }

        let slice = &self.data[offset..];
        let null_pos = slice.iter().position(|&b| b == 0)?;

        let str_slice = &slice[..null_pos];
        std::str::from_utf8(str_slice).ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skeleton_gr2_string_table() {
        let obj = Gr2StringTable::from_buffer(vec![0]);
        assert!(obj.is_ready());
    }
}
