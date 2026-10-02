use m2_client::granny_gr2_binary_parser::{
    parse_magic, parse_header, parse_sections, parse_gr2, Gr2ParseError,
    GrnFileMagicValue, GrnFileHeader, GrnSection
};

#[test]
fn test_parse_magic_valid() {
    let mut data = vec![0u8; 32];
    let magic_bytes: [u32; 4] = [0x29535247, 0x6972614d, 0x67616220, 0x3131796e];
    for (i, &val) in magic_bytes.iter().enumerate() {
        data[i * 4..(i + 1) * 4].copy_from_slice(&val.to_le_bytes());
    }
    // Set header size
    data[16..20].copy_from_slice(&32u32.to_le_bytes());

    let result = parse_magic(&data);
    assert!(result.is_ok());
    let magic = result.unwrap();
    assert_eq!(magic.magic, magic_bytes);
    assert_eq!(magic.header_size, 32);
}

#[test]
fn test_parse_magic_invalid() {
    let data = vec![0u8; 32];
    let result = parse_magic(&data);
    assert_eq!(result.unwrap_err(), Gr2ParseError::InvalidMagic);
}

#[test]
fn test_parse_magic_buffer_too_small() {
    let data = vec![0u8; 15];
    let result = parse_magic(&data);
    assert_eq!(result.unwrap_err(), Gr2ParseError::BufferTooSmall);
}

#[test]
fn test_parse_header_valid() {
    let mut data = vec![0u8; 100];
    
    // Header size = 32
    let mut magic = GrnFileMagicValue::default();
    magic.header_size = 32;

    // Fill valid version (7)
    let version = 7u32;
    data[32..36].copy_from_slice(&version.to_le_bytes());

    let result = parse_header(&data, &magic);
    assert!(result.is_ok());
    let header = result.unwrap();
    assert_eq!(header.version, 7);
}

#[test]
fn test_parse_header_invalid_version() {
    let mut data = vec![0u8; 100];
    
    let mut magic = GrnFileMagicValue::default();
    magic.header_size = 32;

    // Fill invalid version (5)
    let version = 5u32;
    data[32..36].copy_from_slice(&version.to_le_bytes());

    let result = parse_header(&data, &magic);
    assert_eq!(result.unwrap_err(), Gr2ParseError::UnsupportedVersion);
}

#[test]
fn test_parse_sections() {
    let mut data = vec![0u8; 100];
    let mut header = GrnFileHeader::default();
    header.section_array_offset = 50;
    header.section_array_count = 1;

    // Set first format in section
    data[50..54].copy_from_slice(&1u32.to_le_bytes());

    let result = parse_sections(&data, &header);
    assert!(result.is_ok());
    let sections = result.unwrap();
    assert_eq!(sections.len(), 1);
    assert_eq!(sections[0].format, 1);
}

#[test]
fn test_parse_sections_buffer_too_small() {
    let data = vec![0u8; 100];
    let mut header = GrnFileHeader::default();
    header.section_array_offset = 90;
    header.section_array_count = 2; // Needs at least 90 + 2*44 = 178 bytes

    let result = parse_sections(&data, &header);
    assert_eq!(result.unwrap_err(), Gr2ParseError::BufferTooSmall);
}
