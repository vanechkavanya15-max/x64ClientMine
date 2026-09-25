//! Modul reprezentujacy wpis w pliku indeksu (.eix) VFS EterPack.
//! Struktura odpowiada C++ 'TEterPackIndex'.


/// Rozmiar bufora na nazwe pliku w archiwum (.eix).
pub const FILENAME_MAX_LEN: usize = 160;

/// Struktura reprezentujaca naglowek pojedynczego pliku w archiwum EterPack (.eix).
/// Odpowiada C++ `TEterPackIndex` z `#pragma pack(push, 4)`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct EixEntry {
    /// Identyfikator pliku.
    pub id: i32,
    /// Nazwa pliku z terminatorem null.
    pub filename: [u8; FILENAME_MAX_LEN + 1],
    // Rust automatycznie doda 3 bajty paddingu, poniewaz kolejne pole to u32.
    // offset_of(filename) = 4, rozmiar = 161. 4 + 161 = 165. Padding do 168.
    
    /// Hash CRC32 nazwy pliku.
    pub filename_crc: u32,
    /// Rzeczywisty (rozpakowany) rozmiar danych.
    pub real_data_size: i32,
    /// Rozmiar danych w pliku (po kompresji/szyfrowaniu).
    pub data_size: i32,
    /// Hash CRC32 zawartosci (jesli md5 jest wylaczone w C++).
    pub data_crc: u32,
    /// Przesuniecie (offset) poczatku danych w pliku .epk.
    pub data_position: i32,
    /// Typ kompresji (np. NONE = 0, COMPRESS = 1, SECURITY = 2, PANAMA = 3, HYBRIDCRYPT = 4).
    pub compressed_type: i8,
    // Rust doda 3 bajty paddingu na koncu struktury dla zachowania wyrownania (align = 4).
    // offset_of(compressed_type) = 188, rozmiar = 1, calkowity rozmiar struktury = 192.
}

impl Default for EixEntry {
    fn default() -> Self {
        Self {
            id: 0,
            filename: [0; FILENAME_MAX_LEN + 1],
            filename_crc: 0,
            real_data_size: 0,
            data_size: 0,
            data_crc: 0,
            data_position: 0,
            compressed_type: 0,
        }
    }
}

impl EixEntry {
    /// Pobiera nazwe pliku jako ciag znakow Rust (zignorowanie ewentualnych bledow UTF-8).
    pub fn get_filename(&self) -> String {
        // Znajdz pierwsze wystapienie bajtu 0.
        let null_pos = self.filename.iter().position(|&c| c == 0).unwrap_or(self.filename.len());
        String::from_utf8_lossy(&self.filename[..null_pos]).into_owned()
    }
    
    /// Ustawia nazwe pliku. Jesli jest za dluga, zostanie obcieta.
    pub fn set_filename(&mut self, name: &str) {
        let bytes = name.as_bytes();
        let len = bytes.len().min(FILENAME_MAX_LEN);
        self.filename[..len].copy_from_slice(&bytes[..len]);
        self.filename[len] = 0; // null terminator
    }
}
