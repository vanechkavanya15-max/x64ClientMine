use m2_client_bridge::vfs::ffi_bridge::*;
use std::ffi::CString;
use std::fs::File;
use std::io::Write;
use std::ptr;

#[test]
fn test_m2_vfs_exists_and_load_file() {
    let test_file_path = "test_vfs_temp.txt";
    let test_content = b"Hello, Metin2 VFS!";
    
    // Create temp file
    {
        let mut file = File::create(test_file_path).unwrap();
        file.write_all(test_content).unwrap();
    }
    
    let c_path = CString::new(test_file_path).unwrap();
    
    // Test exists
    let exists = m2_vfs_exists(c_path.as_ptr());
    assert!(exists, "Plik powinien istniec.");
    
    let non_existent_path = CString::new("non_existent_file_xyz.txt").unwrap();
    let exists_false = m2_vfs_exists(non_existent_path.as_ptr());
    assert!(!exists_false, "Plik nie powinien istniec.");
    
    // Test load file
    let mut out_size: usize = 0;
    let data = m2_vfs_load_file(c_path.as_ptr(), &mut out_size);
    
    assert!(!data.is_null(), "Wskaznik do danych nie moze byc null.");
    assert_eq!(out_size, test_content.len(), "Rozmiar wczytanego pliku sie nie zgadza.");
    
    // Check content
    let loaded_slice = unsafe { std::slice::from_raw_parts(data, out_size) };
    assert_eq!(loaded_slice, test_content, "Zawartosc pliku sie nie zgadza.");
    
    // Free memory
    m2_vfs_free(data, out_size);
    
    // Cleanup temp file
    std::fs::remove_file(test_file_path).unwrap();
}

#[test]
fn test_m2_vfs_null_pointers() {
    // exists
    let exists_null = m2_vfs_exists(ptr::null());
    assert!(!exists_null, "Przekazanie null do m2_vfs_exists powinno zwrocic false.");
    
    // load file
    let mut out_size: usize = 0;
    let data_null_path = m2_vfs_load_file(ptr::null(), &mut out_size);
    assert!(data_null_path.is_null());
    
    let c_path = CString::new("test.txt").unwrap();
    let data_null_size = m2_vfs_load_file(c_path.as_ptr(), ptr::null_mut());
    assert!(data_null_size.is_null());
    
    // free
    // Should not crash
    m2_vfs_free(ptr::null_mut(), 0);
}
