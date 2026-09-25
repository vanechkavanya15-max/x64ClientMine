use crate::ffi_guard::safe_ffi_call;
use std::slice;

// Structura reprezentujaca naglowek pliku .eix
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EixHeader {
    pub magic: u32,       // c_PackCC = 'EPKD'
    pub version: u32,     // Wersja, domyslnie 2
    pub file_count: u32,  // Liczba plikow (m_indexCount)
}

impl EixHeader {
    // Stala reprezentujaca 4-bajtowy kod EPKD
    pub const MAGIC_EPKD: u32 = 0x444B5045; // 'D' | 'K' << 8 | 'P' << 16 | 'E' << 24
    pub const HEADER_SIZE: usize = 12;

    // Funkcja do parsowania naglowka z bufora bajtow
    pub fn parse(buffer: &[u8]) -> Option<Self> {
        if buffer.len() < Self::HEADER_SIZE {
            return None;
        }

        // Dekodowanie pol typu little-endian
        let magic = u32::from_le_bytes(buffer[0..4].try_into().unwrap());
        let version = u32::from_le_bytes(buffer[4..8].try_into().unwrap());
        let file_count = u32::from_le_bytes(buffer[8..12].try_into().unwrap());

        // Sprawdzanie poprawnosci naglowka
        if magic != Self::MAGIC_EPKD {
            return None;
        }

        Some(EixHeader {
            magic,
            version,
            file_count,
        })
    }
}

// Funkcja eksportowana do C FFI
// Parsuje naglowek .eix z bufora wejsciowego i zapisuje wynik do struktury docelowej
// Zwraca 1 jesli parsowanie sie powiodlo, 0 w przeciwnym wypadku.
#[no_mangle]
pub extern "C" fn m2_eix_header_parse(
    buf: *const u8,
    len: usize,
    out_header: *mut EixHeader,
) -> u32 {
    safe_ffi_call(0, || {
        if buf.is_null() || out_header.is_null() || len < EixHeader::HEADER_SIZE {
            return 0;
        }

        let slice = unsafe { slice::from_raw_parts(buf, len) };
        
        match EixHeader::parse(slice) {
            Some(header) => {
                unsafe { *out_header = header; }
                1
            },
            None => 0,
        }
    })
}
