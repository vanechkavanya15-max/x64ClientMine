use m2_client_bridge::vfs::eix_header::{EixHeader, m2_eix_header_parse};

#[test]
fn test_eix_header_parse_valid() {
    // Naglowek: 'EPKD' (0x444B5045), wersja (2), liczba plikow (10)
    let buffer = [
        0x45, 0x50, 0x4B, 0x44, // magic
        0x02, 0x00, 0x00, 0x00, // version
        0x0A, 0x00, 0x00, 0x00, // file_count
    ];

    let header_opt = EixHeader::parse(&buffer);
    assert!(header_opt.is_some());
    let header = header_opt.unwrap();
    assert_eq!(header.magic, EixHeader::MAGIC_EPKD);
    assert_eq!(header.version, 2);
    assert_eq!(header.file_count, 10);
}

#[test]
fn test_eix_header_parse_invalid_magic() {
    let buffer = [
        0x41, 0x42, 0x43, 0x44, // invalid magic
        0x02, 0x00, 0x00, 0x00,
        0x0A, 0x00, 0x00, 0x00,
    ];

    let header_opt = EixHeader::parse(&buffer);
    assert!(header_opt.is_none());
}

#[test]
fn test_eix_header_parse_short_buffer() {
    let buffer = [
        0x45, 0x50, 0x4B, 0x44,
        0x02, 0x00, 0x00, 0x00,
        // brakuje 4 bajtow
    ];

    let header_opt = EixHeader::parse(&buffer);
    assert!(header_opt.is_none());
}

#[test]
fn test_ffi_eix_header_parse() {
    let buffer = [
        0x45, 0x50, 0x4B, 0x44, // magic
        0x02, 0x00, 0x00, 0x00, // version
        0x0A, 0x00, 0x00, 0x00, // file_count
    ];

    let mut header = EixHeader {
        magic: 0,
        version: 0,
        file_count: 0,
    };

    let result = m2_eix_header_parse(buffer.as_ptr(), buffer.len(), &mut header);
    
    assert_eq!(result, 1);
    assert_eq!(header.magic, EixHeader::MAGIC_EPKD);
    assert_eq!(header.version, 2);
    assert_eq!(header.file_count, 10);
}

#[test]
fn test_ffi_eix_header_parse_invalid_ptr() {
    let buffer = [
        0x45, 0x50, 0x4B, 0x44,
        0x02, 0x00, 0x00, 0x00,
        0x0A, 0x00, 0x00, 0x00,
    ];
    let mut header = EixHeader {
        magic: 0,
        version: 0,
        file_count: 0,
    };

    // test null pointers
    assert_eq!(m2_eix_header_parse(std::ptr::null(), buffer.len(), &mut header), 0);
    assert_eq!(m2_eix_header_parse(buffer.as_ptr(), buffer.len(), std::ptr::null_mut()), 0);
    assert_eq!(m2_eix_header_parse(buffer.as_ptr(), 10, &mut header), 0); // Zbyt krotki bufor
}
