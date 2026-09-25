use m2_client_granny::gr2_header::{Gr2Header, Gr2HeaderError, Gr2Version, HEADER_SIZE, MAGIC_LITTLE_ENDIAN_32};

#[test]
fn test_integration_gr2_header() {
    let instance = Gr2Header::new();
    assert!(instance.is_ready());
}

#[test]
fn test_integration_parse_buffer_too_short() {
    let buffer = vec![0u8; HEADER_SIZE - 1];
    let result = Gr2Header::parse(&buffer);
    assert_eq!(result.unwrap_err(), Gr2HeaderError::BufferTooShort);
}

#[test]
fn test_integration_parse_invalid_magic() {
    let buffer = vec![0x00; HEADER_SIZE];
    let result = Gr2Header::parse(&buffer);
    assert_eq!(result.unwrap_err(), Gr2HeaderError::InvalidMagic);
}

#[test]
fn test_integration_parse_valid_le() {
    let mut buffer = vec![0u8; HEADER_SIZE + 20]; // 32 bytes for magic block, plus 20 for basic grn_file_header
    
    // Little Endian 32-bit Magic
    let magic_bytes: Vec<u8> = MAGIC_LITTLE_ENDIAN_32
        .iter()
        .flat_map(|&val| val.to_le_bytes().to_vec())
        .collect();
    buffer[0..16].copy_from_slice(&magic_bytes);

    // Header Size
    let header_size: u32 = 32;
    buffer[16..20].copy_from_slice(&header_size.to_le_bytes());

    // Header Format
    let header_format: u32 = 1; // Version 2.9
    buffer[20..24].copy_from_slice(&header_format.to_le_bytes());

    // Section array offset
    let sec_arr_offset: u32 = 1234;
    buffer[32 + 12 .. 32 + 16].copy_from_slice(&sec_arr_offset.to_le_bytes());
    
    // Section array count
    let sec_arr_count: u32 = 5;
    buffer[32 + 16 .. 32 + 20].copy_from_slice(&sec_arr_count.to_le_bytes());

    let result = Gr2Header::parse(&buffer);
    assert!(result.is_ok());
    let header = result.unwrap();
    
    assert_eq!(header.magic, MAGIC_LITTLE_ENDIAN_32);
    assert_eq!(header.is_little_endian, true);
    assert_eq!(header.header_size, 32);
    assert_eq!(header.header_format, 1);
    assert_eq!(header.version, Gr2Version::Version29);
    assert_eq!(header.section_array_offset, 1234);
    assert_eq!(header.section_array_count, 5);
}
