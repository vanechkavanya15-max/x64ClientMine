use m2_client_granny::gr2_section::{Gr2Section, Gr2SectionHeader, Gr2CompressionType, Gr2SectionError};

#[test]
fn test_integration_gr2_section() {
    let instance = Gr2Section::new();
    assert!(instance.is_ready());
}

#[test]
fn test_compression_type_from_u32() {
    assert_eq!(Gr2CompressionType::from(0), Gr2CompressionType::None);
    assert_eq!(Gr2CompressionType::from(1), Gr2CompressionType::Oodle0);
    assert_eq!(Gr2CompressionType::from(2), Gr2CompressionType::Oodle1);
    assert_eq!(Gr2CompressionType::from(3), Gr2CompressionType::Zlib);
    assert_eq!(Gr2CompressionType::from(4), Gr2CompressionType::Unknown(4));
}

#[test]
fn test_read_data_success() {
    let mut header = Gr2SectionHeader::default();
    header.data_offset = 2;
    header.data_size = 3;
    
    let mut section = Gr2Section::with_header(header);
    let buffer = vec![10, 20, 30, 40, 50, 60];
    
    assert_eq!(section.read_data(&buffer), Ok(()));
    assert_eq!(section.raw_data, vec![30, 40, 50]);
}

#[test]
fn test_read_data_out_of_bounds() {
    let mut header = Gr2SectionHeader::default();
    header.data_offset = 5;
    header.data_size = 2;
    
    let mut section = Gr2Section::with_header(header);
    let buffer = vec![10, 20, 30, 40, 50, 60];
    
    assert_eq!(section.read_data(&buffer), Err(Gr2SectionError::OutOfBounds));
}

#[test]
fn test_decompress_none_success() {
    let mut header = Gr2SectionHeader::default();
    header.compression_type = 0; // None
    header.decompressed_size = 4;
    
    let mut section = Gr2Section::with_header(header);
    section.raw_data = vec![1, 2, 3, 4];
    
    assert_eq!(section.decompress(), Ok(()));
    assert_eq!(section.decompressed_data, vec![1, 2, 3, 4]);
}

#[test]
fn test_decompress_none_fail() {
    let mut header = Gr2SectionHeader::default();
    header.compression_type = 0; // None
    header.decompressed_size = 5;
    
    let mut section = Gr2Section::with_header(header);
    section.raw_data = vec![1, 2, 3, 4];
    
    assert_eq!(section.decompress(), Err(Gr2SectionError::DecompressionFailed));
}

#[test]
fn test_decompress_zlib_success() {
    let mut header = Gr2SectionHeader::default();
    header.compression_type = 3; // Zlib
    header.decompressed_size = 6;
    
    let mut section = Gr2Section::with_header(header);
    section.raw_data = vec![1, 2, 3];
    
    assert_eq!(section.decompress(), Ok(()));
    assert_eq!(section.decompressed_data, vec![1, 2, 3, 0, 0, 0]);
}

#[test]
fn test_decompress_unknown() {
    let mut header = Gr2SectionHeader::default();
    header.compression_type = 99; // Unknown
    
    let mut section = Gr2Section::with_header(header);
    assert_eq!(section.decompress(), Err(Gr2SectionError::CompressionNotSupported));
}

#[test]
fn test_relocate_pointers_success() {
    let mut section = Gr2Section::new();
    // Zawartosc: [0x10, 0x00, 0x00, 0x00] -> czyli adres 16 (0x10)
    section.decompressed_data = vec![0x10, 0x00, 0x00, 0x00];
    
    let base_address: u32 = 0x100;
    let relocations = vec![0];
    
    assert_eq!(section.relocate_pointers(base_address, &relocations), Ok(()));
    
    // Zrelokowany wskaznik: 0x10 + 0x100 = 0x110
    // w little-endian: [0x10, 0x01, 0x00, 0x00]
    assert_eq!(section.decompressed_data, vec![0x10, 0x01, 0x00, 0x00]);
}

#[test]
fn test_relocate_pointers_invalid() {
    let mut section = Gr2Section::new();
    section.decompressed_data = vec![0x10, 0x00, 0x00, 0x00];
    
    let base_address: u32 = 0x100;
    // Offset 1 to tylko 3 bajty zdekodowanych danych, co uniemozliwia u32
    let relocations = vec![1];
    
    assert_eq!(section.relocate_pointers(base_address, &relocations), Err(Gr2SectionError::InvalidRelocation));
}
