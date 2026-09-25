use m2_client_bridge::vfs::hybrid_crypt::HybridCrypt;
use m2_client_bridge::vfs::hybrid_crypt::{m2_hybrid_crypt_new, m2_hybrid_crypt_free, m2_hybrid_crypt_read_sdb, m2_hybrid_crypt_get_sdb};

#[test]
fn test_read_sdb_success() {
    let mut crypt = HybridCrypt::new();
    
    // Test stream: 
    // - Hash: 0x12345678 (4 bytes)
    // - Size: 5 (1 byte)
    // - Data: [1, 2, 3, 4, 5] (5 bytes)
    let mut stream = vec![];
    stream.extend_from_slice(&0x12345678u32.to_ne_bytes());
    stream.push(5);
    stream.extend_from_slice(&[1, 2, 3, 4, 5]);
    
    // Add extra data to test offset
    stream.extend_from_slice(&[9, 9, 9]);

    let offset = crypt.read_sdb_from_stream(&stream).expect("Failed to read SDB");
    
    assert_eq!(offset, 4 + 1 + 5);
    
    let sdb = crypt.get_sdb(0x12345678).expect("SDB not found");
    assert_eq!(sdb, &[1, 2, 3, 4, 5]);
}

#[test]
fn test_get_sdb_not_found() {
    let crypt = HybridCrypt::new();
    assert!(crypt.get_sdb(0x12345678).is_none());
}

#[test]
fn test_ffi_lifecycle() {
    let ptr = m2_hybrid_crypt_new();
    assert!(!ptr.is_null());
    
    // Should not panic or crash
    m2_hybrid_crypt_free(ptr);
    m2_hybrid_crypt_free(std::ptr::null_mut()); // Test null pointer safe handling
}

#[test]
fn test_ffi_read_and_get_sdb() {
    let ptr = m2_hybrid_crypt_new();
    assert!(!ptr.is_null());
    
    let mut stream = vec![];
    stream.extend_from_slice(&0xABCDEF01u32.to_ne_bytes());
    stream.push(3);
    stream.extend_from_slice(&[42, 43, 44]);
    
    let read_len = m2_hybrid_crypt_read_sdb(ptr, stream.as_ptr(), stream.len());
    assert_eq!(read_len, 8); // 4 + 1 + 3
    
    let mut out_sdb: *const u8 = std::ptr::null();
    let mut out_size: usize = 0;
    
    let found = m2_hybrid_crypt_get_sdb(ptr, 0xABCDEF01, &mut out_sdb, &mut out_size);
    assert!(found);
    assert!(!out_sdb.is_null());
    assert_eq!(out_size, 3);
    
    let slice = unsafe { std::slice::from_raw_parts(out_sdb, out_size) };
    assert_eq!(slice, &[42, 43, 44]);
    
    // Test not found
    let found = m2_hybrid_crypt_get_sdb(ptr, 0x12345678, &mut out_sdb, &mut out_size);
    assert!(!found);
    
    // Test invalid args
    assert_eq!(m2_hybrid_crypt_read_sdb(std::ptr::null_mut(), stream.as_ptr(), stream.len()), -1);
    assert_eq!(m2_hybrid_crypt_read_sdb(ptr, std::ptr::null(), stream.len()), -1);
    assert_eq!(m2_hybrid_crypt_read_sdb(ptr, stream.as_ptr(), 0), -1);
    
    let mut out_sdb: *const u8 = std::ptr::null();
    let mut out_size: usize = 0;
    assert!(!m2_hybrid_crypt_get_sdb(std::ptr::null_mut(), 0xABCDEF01, &mut out_sdb, &mut out_size));
    assert!(!m2_hybrid_crypt_get_sdb(ptr, 0xABCDEF01, std::ptr::null_mut(), &mut out_size));
    assert!(!m2_hybrid_crypt_get_sdb(ptr, 0xABCDEF01, &mut out_sdb, std::ptr::null_mut()));

    m2_hybrid_crypt_free(ptr);
}
