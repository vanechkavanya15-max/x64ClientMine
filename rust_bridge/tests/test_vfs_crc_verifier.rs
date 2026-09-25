use m2_client_bridge::vfs::crc_verifier::m2_vfs_verify_crc32;
use m2_client_bridge::crc32::get_crc32;

#[test]
fn test_m2_vfs_verify_crc32_valid() {
    let buf = b"Testowanie mostka Rust dla Metin2!";
    let expected_crc = get_crc32(buf);
    
    let result = m2_vfs_verify_crc32(buf.as_ptr(), buf.len(), expected_crc);
    assert_eq!(result, 1, "Weryfikacja prawidlowego CRC musi zwrocic 1");
}

#[test]
fn test_m2_vfs_verify_crc32_invalid() {
    let buf = b"Testowanie mostka Rust dla Metin2!";
    let expected_crc = 0xDEADBEEF;
    
    let result = m2_vfs_verify_crc32(buf.as_ptr(), buf.len(), expected_crc);
    assert_eq!(result, 0, "Weryfikacja blednego CRC musi zwrocic 0");
}

#[test]
fn test_m2_vfs_verify_crc32_null() {
    let result = m2_vfs_verify_crc32(std::ptr::null(), 100, 0x12345678);
    assert_eq!(result, 0, "Weryfikacja bufora null musi zwrocic 0 i nie panikowac");
}

#[test]
fn test_m2_vfs_verify_crc32_empty() {
    let buf = b"";
    let expected_crc = 0; // pusty bufor ma CRC 0
    let result = m2_vfs_verify_crc32(buf.as_ptr(), buf.len(), expected_crc);
    assert_eq!(result, 1, "Weryfikacja pustego bufora z CRC=0 musi zwrocic 1");
    
    let result_invalid = m2_vfs_verify_crc32(buf.as_ptr(), buf.len(), 0x12345678);
    assert_eq!(result_invalid, 0, "Weryfikacja pustego bufora z niezerowym CRC musi zwrocic 0");
}
