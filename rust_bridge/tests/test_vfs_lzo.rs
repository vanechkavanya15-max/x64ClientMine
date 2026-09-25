use m2_client_bridge::vfs::lzo::{LzoDecompressor, VfsError, m2_vfs_decompress_lzo};

#[test]
fn test_lzo_decompress_success() {
    let uncompressed_data = b"A".repeat(100);
    let compressed_data: [u8; 9] = [18, 65, 32, 66, 0, 0, 17, 0, 0];
    
    let result = LzoDecompressor::decompress(&compressed_data, 100);
    assert!(result.is_ok(), "Decompression failed");
    assert_eq!(result.unwrap(), uncompressed_data);
}

#[test]
fn test_lzo_decompress_invalid_data() {
    let invalid_data: [u8; 5] = [255, 255, 255, 255, 255];
    let result = LzoDecompressor::decompress(&invalid_data, 100);
    assert_eq!(result, Err(VfsError::InvalidData));
}

#[test]
fn test_lzo_decompress_size_mismatch() {
    let compressed_data: [u8; 9] = [18, 65, 32, 66, 0, 0, 17, 0, 0];
    let result = LzoDecompressor::decompress(&compressed_data, 50);
    assert!(result == Err(VfsError::SizeMismatch) || result == Err(VfsError::InvalidData));
}

#[test]
fn test_m2_vfs_decompress_lzo_success() {
    let uncompressed_data = b"A".repeat(100);
    let compressed_data: [u8; 9] = [18, 65, 32, 66, 0, 0, 17, 0, 0];
    let mut dst = vec![0u8; 100];
    
    let success = unsafe {
        m2_vfs_decompress_lzo(
            compressed_data.as_ptr(),
            compressed_data.len() as u32,
            dst.as_mut_ptr(),
            dst.len() as u32,
        )
    };
    
    assert!(success);
    assert_eq!(dst, uncompressed_data);
}

#[test]
fn test_m2_vfs_decompress_lzo_null_pointers() {
    let mut dst = vec![0u8; 100];
    let success = unsafe {
        m2_vfs_decompress_lzo(
            std::ptr::null(),
            9,
            dst.as_mut_ptr(),
            100,
        )
    };
    assert!(!success);
    
    let compressed_data: [u8; 9] = [18, 65, 32, 66, 0, 0, 17, 0, 0];
    let success = unsafe {
        m2_vfs_decompress_lzo(
            compressed_data.as_ptr(),
            9,
            std::ptr::null_mut(),
            100,
        )
    };
    assert!(!success);
}
