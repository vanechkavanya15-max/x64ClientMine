use m2_client_bridge::vfs::eix_entry::{TEixEntry, VfsError, m2_vfs_parse_entry, m2_vfs_alloc_eix_entry, m2_vfs_free_eix_entry};
use std::mem;

#[test]
fn test_eix_entry_layout() {
    // Sprawdz rozmiar i wyrownanie struktury (musi odpowiadac #pragma pack(1) i specyfikacji)
    assert_eq!(mem::size_of::<TEixEntry>(), 24, "Rozmiar TEixEntry musi wynosic 24 bajty");
    assert_eq!(mem::align_of::<TEixEntry>(), 1, "Wyrownanie TEixEntry musi wynosic 1 bajt (packed)");
}

#[test]
fn test_eix_entry_from_bytes_success() {
    // 24 bajty poprawnych danych
    let data: [u8; 24] = [
        0x01, 0x00, 0x00, 0x00, // index = 1
        0x12, 0x34, 0x56, 0x78, // filename_crc
        0x00, 0x04, 0x00, 0x00, // real_data_size = 1024
        0x00, 0x02, 0x00, 0x00, // compressed_data_size = 512
        0xAB, 0xCD, 0xEF, 0x01, // data_crc
        0x00, 0x10, 0x00, 0x00, // data_offset = 4096
    ];

    let entry = TEixEntry::from_bytes(&data).unwrap();
    
    // Copy fields before asserting to avoid unaligned reference errors
    let index = entry.index;
    let filename_crc = entry.filename_crc;
    let real_data_size = entry.real_data_size;
    let compressed_data_size = entry.compressed_data_size;
    let data_crc = entry.data_crc;
    let data_offset = entry.data_offset;

    assert_eq!(index, 1);
    assert_eq!(filename_crc, 0x78563412);
    assert_eq!(real_data_size, 1024);
    assert_eq!(compressed_data_size, 512);
    assert_eq!(data_crc, 0x01EFCDAB);
    assert_eq!(data_offset, 4096);
}

#[test]
fn test_eix_entry_from_bytes_buffer_too_small() {
    let data: [u8; 23] = [0; 23];
    let result = TEixEntry::from_bytes(&data);
    assert_eq!(result.is_err(), true);
    assert_eq!(result.err().unwrap(), VfsError::BufferTooSmall);
}

#[test]
fn test_eix_entry_ffi() {
    let data: [u8; 24] = [
        0x02, 0x00, 0x00, 0x00, // index = 2
        0x12, 0x34, 0x56, 0x78, // filename_crc
        0x00, 0x04, 0x00, 0x00, // real_data_size = 1024
        0x00, 0x02, 0x00, 0x00, // compressed_data_size = 512
        0xAB, 0xCD, 0xEF, 0x01, // data_crc
        0x00, 0x10, 0x00, 0x00, // data_offset = 4096
    ];

    unsafe {
        let entry_ptr = m2_vfs_alloc_eix_entry();
        assert!(!entry_ptr.is_null());

        let success = m2_vfs_parse_entry(data.as_ptr(), data.len(), entry_ptr);
        assert!(success);

        let entry = *entry_ptr;
        let index = entry.index;
        let real_data_size = entry.real_data_size;
        let data_offset = entry.data_offset;

        assert_eq!(index, 2);
        assert_eq!(real_data_size, 1024);
        assert_eq!(data_offset, 4096);

        m2_vfs_free_eix_entry(entry_ptr);
    }
}

#[test]
fn test_eix_entry_ffi_null_pointers() {
    unsafe {
        let mut entry = TEixEntry::default();
        let success = m2_vfs_parse_entry(std::ptr::null(), 24, &mut entry);
        assert!(!success);

        let data: [u8; 24] = [0; 24];
        let success = m2_vfs_parse_entry(data.as_ptr(), data.len(), std::ptr::null_mut());
        assert!(!success);
    }
}
