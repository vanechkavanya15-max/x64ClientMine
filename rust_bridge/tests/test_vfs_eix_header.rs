use m2_client_bridge::vfs::eix_header::{EPKD_MAGIC, TEixHeader, VfsError, m2_vfs_parse_eix_header};

#[test]
fn test_eix_header_parse_valid() {
    // Naglowek: 'EPKD' (0x444B5045), wersja (2), liczba plikow (10)
    let buffer = [
        0x45, 0x50, 0x4B, 0x44, // magic
        0x02, 0x00, 0x00, 0x00, // version
        0x0A, 0x00, 0x00, 0x00, // file_count
    ];

    let header_opt = TEixHeader::from_bytes(&buffer);
    assert!(header_opt.is_ok());
    let header = header_opt.unwrap();
    let magic = header.magic;
    assert_eq!(magic, EPKD_MAGIC);
    let version = header.version;
    assert_eq!(version, 2);
    let file_count = header.file_count;
    assert_eq!(file_count, 10);
    assert!(header.is_valid());
}

#[test]
fn test_eix_header_parse_invalid_magic() {
    let buffer = [
        0x41, 0x42, 0x43, 0x44, // invalid magic
        0x02, 0x00, 0x00, 0x00,
        0x0A, 0x00, 0x00, 0x00,
    ];

    let header_opt = TEixHeader::from_bytes(&buffer);
    assert_eq!(header_opt, Err(VfsError::InvalidMagic));
}

#[test]
fn test_eix_header_parse_short_buffer() {
    let buffer = [
        0x45, 0x50, 0x4B, 0x44,
        0x02, 0x00, 0x00, 0x00,
        // brakuje 4 bajtow
    ];

    let header_opt = TEixHeader::from_bytes(&buffer);
    assert_eq!(header_opt, Err(VfsError::BufferTooShort));
}

#[test]
fn test_ffi_eix_header_parse() {
    let buffer = [
        0x45, 0x50, 0x4B, 0x44, // magic
        0x02, 0x00, 0x00, 0x00, // version
        0x0A, 0x00, 0x00, 0x00, // file_count
    ];

    let mut count = 0;

    let result = unsafe { m2_vfs_parse_eix_header(buffer.as_ptr(), buffer.len() as u32, &mut count) };
    
    assert_eq!(result, true);
    assert_eq!(count, 10);
}

#[test]
fn test_ffi_eix_header_parse_invalid_ptr() {
    let buffer = [
        0x45, 0x50, 0x4B, 0x44,
        0x02, 0x00, 0x00, 0x00,
        0x0A, 0x00, 0x00, 0x00,
    ];
    
    let mut count = 0;

    // test null pointers
    assert_eq!(unsafe { m2_vfs_parse_eix_header(std::ptr::null(), buffer.len() as u32, &mut count) }, false);
    assert_eq!(unsafe { m2_vfs_parse_eix_header(buffer.as_ptr(), buffer.len() as u32, std::ptr::null_mut()) }, false);
    assert_eq!(unsafe { m2_vfs_parse_eix_header(buffer.as_ptr(), 10, &mut count) }, false); // Zbyt krotki bufor
}
