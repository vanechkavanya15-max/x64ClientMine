//! Modul gr2_header dla skrzynki m2-client-granny
//! Parser naglowka pliku binarnego Granny 2 (.gr2)
//! Weryfikacja sygnatury magic, wersji formatu (2.4/2.9), endianness i offsetow sekcji

use std::convert::TryInto;

/// Oczekiwany rozmiar glownego naglowka (Magic Value)
pub const HEADER_SIZE: usize = 32;

/// Sygnatury magiczne
pub const MAGIC_LITTLE_ENDIAN_32: [u32; 4] = [0x82317529, 0x04670516, 0x0a48108e, 0x3e4dfa16];
pub const MAGIC_BIG_ENDIAN_32: [u32; 4] = [0x29753182, 0x16056704, 0x8e10480a, 0x16fa4d3e];
pub const MAGIC_LITTLE_ENDIAN_64: [u32; 4] = [0x8231752b, 0x04670516, 0x0a48108e, 0x3e4dfa16];
pub const MAGIC_BIG_ENDIAN_64: [u32; 4] = [0x2b753182, 0x16056704, 0x8e10480a, 0x16fa4d3e];

/// Reprezentuje wersje formatu pliku Granny 2 (np. 2.4 lub 2.9)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gr2Version {
    Version24,
    Version29,
    Unknown,
}

impl Default for Gr2Version {
    fn default() -> Self {
        Gr2Version::Unknown
    }
}

/// Reprezentuje bledy podczas parsowania naglowka
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Gr2HeaderError {
    BufferTooShort,
    InvalidMagic,
    UnsupportedVersion,
}

/// Glowna struktura implementacyjna
#[derive(Debug, Clone, Default)]
pub struct Gr2Header {
    pub initialized: bool,
    pub magic: [u32; 4],
    pub version: Gr2Version,
    pub is_little_endian: bool,
    pub header_size: u32,
    pub header_format: u32,
    
    // Z polstruktury granny_grn_file_header
    pub section_array_offset: u32,
    pub section_array_count: u32,
}

impl Gr2Header {
    pub fn new() -> Self {
        Self {
            initialized: true,
            magic: [0; 4],
            version: Gr2Version::Unknown,
            is_little_endian: true,
            header_size: 0,
            header_format: 0,
            section_array_offset: 0,
            section_array_count: 0,
        }
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }

    /// Parsuje naglowek z bufora bajtow
    pub fn parse(buffer: &[u8]) -> Result<Self, Gr2HeaderError> {
        if buffer.len() < HEADER_SIZE {
            return Err(Gr2HeaderError::BufferTooShort);
        }

        // Czytamy magic, zakladajac little-endian lub big-endian.
        let m0_le = u32::from_le_bytes(buffer[0..4].try_into().unwrap());
        let m0_be = u32::from_be_bytes(buffer[0..4].try_into().unwrap());
        
        let is_le = if m0_le == MAGIC_LITTLE_ENDIAN_32[0] || m0_le == MAGIC_LITTLE_ENDIAN_64[0] {
            true
        } else if m0_be == MAGIC_BIG_ENDIAN_32[0] || m0_be == MAGIC_BIG_ENDIAN_64[0] {
            false
        } else {
            return Err(Gr2HeaderError::InvalidMagic);
        };

        let mut magic = [0u32; 4];
        let mut offset = 0;
        for i in 0..4 {
            let chunk: [u8; 4] = buffer[offset..offset+4].try_into().unwrap();
            magic[i] = if is_le { u32::from_le_bytes(chunk) } else { u32::from_be_bytes(chunk) };
            offset += 4;
        }

        // Weryfikacja calego magic
        if magic != MAGIC_LITTLE_ENDIAN_32 && magic != MAGIC_BIG_ENDIAN_32 &&
           magic != MAGIC_LITTLE_ENDIAN_64 && magic != MAGIC_BIG_ENDIAN_64 {
            return Err(Gr2HeaderError::InvalidMagic);
        }

        // Odczyt formatu naglowka z granny_grn_file_magic_value
        let chunk_size: [u8; 4] = buffer[offset..offset+4].try_into().unwrap();
        let header_size = if is_le { u32::from_le_bytes(chunk_size) } else { u32::from_be_bytes(chunk_size) };
        offset += 4;
        
        let chunk_fmt: [u8; 4] = buffer[offset..offset+4].try_into().unwrap();
        let header_format = if is_le { u32::from_le_bytes(chunk_fmt) } else { u32::from_be_bytes(chunk_fmt) };
        
        // Pomin Reserved[2]
        // offset += 8;
        // The structure granny_grn_file_header starts right after header_size bytes according to Granny logic,
        // but typically header_size is the size of the magic block (32 bytes).

        // Okreslenie wersji
        let version = match header_format {
            0 => Gr2Version::Version24,
            1 => Gr2Version::Version29, // mapowanie, mozna dostosowac zaleznie od rzeczywistych wartosci Granny 2
            _ => Gr2Version::Unknown,
        };
        
        let mut section_array_offset = 0;
        let mut section_array_count = 0;

        // Header size specifies where the granny_grn_file_header begins, usually 32 bytes from file start
        let grn_header_offset = header_size as usize;
        let grn_header_size_needed = 4 + 4 + 4 + 4 + 4; // Zabezpieczenie minimalnego wymiaru (Version, TotalSize, CRC, SectionArrayOffset, SectionArrayCount)

        if buffer.len() >= grn_header_offset + grn_header_size_needed {
            let sec_arr_off = grn_header_offset + 12;
            let sec_arr_cnt_off = grn_header_offset + 16;
            
            let chunk_sec_off: [u8; 4] = buffer[sec_arr_off..sec_arr_off+4].try_into().unwrap();
            section_array_offset = if is_le { u32::from_le_bytes(chunk_sec_off) } else { u32::from_be_bytes(chunk_sec_off) };

            let chunk_sec_cnt: [u8; 4] = buffer[sec_arr_cnt_off..sec_arr_cnt_off+4].try_into().unwrap();
            section_array_count = if is_le { u32::from_le_bytes(chunk_sec_cnt) } else { u32::from_be_bytes(chunk_sec_cnt) };
        }

        let header = Self {
            initialized: true,
            magic,
            version,
            is_little_endian: is_le,
            header_size,
            header_format,
            section_array_offset,
            section_array_count,
        };

        Ok(header)
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

    #[test]
    fn test_parse_buffer_too_short() {
        let buffer = vec![0u8; 10];
        let result = Gr2Header::parse(&buffer);
        assert_eq!(result.unwrap_err(), Gr2HeaderError::BufferTooShort);
    }

    #[test]
    fn test_parse_invalid_magic() {
        let buffer = vec![0xFF; HEADER_SIZE];
        let result = Gr2Header::parse(&buffer);
        assert_eq!(result.unwrap_err(), Gr2HeaderError::InvalidMagic);
    }

    #[test]
    fn test_parse_valid_le() {
        let mut buffer = vec![0u8; HEADER_SIZE + 20];
        // Little Endian 32-bit Magic
        let magic_bytes: Vec<u8> = MAGIC_LITTLE_ENDIAN_32
            .iter()
            .flat_map(|&val| val.to_le_bytes().to_vec())
            .collect();
        buffer[0..16].copy_from_slice(&magic_bytes);

        // Header Size - 32 indicates that granny_grn_file_header starts at offset 32
        let header_size: u32 = 32;
        buffer[16..20].copy_from_slice(&header_size.to_le_bytes());

        // Header Format
        let header_format: u32 = 0; // Version 2.4
        buffer[20..24].copy_from_slice(&header_format.to_le_bytes());

        // Offset 32 to start of granny_grn_file_header
        // We write section_array_offset and count
        let sec_arr_offset: u32 = 1234;
        buffer[32 + 12 .. 32 + 16].copy_from_slice(&sec_arr_offset.to_le_bytes());
        let sec_arr_count: u32 = 5;
        buffer[32 + 16 .. 32 + 20].copy_from_slice(&sec_arr_count.to_le_bytes());

        let result = Gr2Header::parse(&buffer);
        assert!(result.is_ok());
        let header = result.unwrap();
        assert_eq!(header.magic, MAGIC_LITTLE_ENDIAN_32);
        assert_eq!(header.is_little_endian, true);
        assert_eq!(header.header_size, 32);
        assert_eq!(header.header_format, 0);
        assert_eq!(header.version, Gr2Version::Version24);
        assert_eq!(header.section_array_offset, 1234);
        assert_eq!(header.section_array_count, 5);
    }
}
