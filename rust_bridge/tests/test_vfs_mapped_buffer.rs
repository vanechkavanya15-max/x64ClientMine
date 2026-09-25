use m2_client_bridge::vfs::mapped_buffer::{MappedBuffer, m2_vfs_free_buffer};
use std::ptr;

#[test]
fn test_mapped_buffer_creation_and_raw_conversion() {
    let vec = vec![1, 2, 3, 4, 5];
    let original_len = vec.len();
    
    let buffer = MappedBuffer::new(vec);
    let (ptr, size) = buffer.into_raw();
    
    assert!(!ptr.is_null());
    assert_eq!(size, original_len);
    
    // Odtwarzamy bufor przez from_raw i weryfikujemy zawartosc
    unsafe {
        let restored_buffer = MappedBuffer::from_raw(ptr, size);
        let restored_data = restored_buffer.into_raw();
        let slice = std::slice::from_raw_parts(restored_data.0, restored_data.1);
        assert_eq!(slice, &[1, 2, 3, 4, 5]);
        
        // Zwalniamy odtworzony bufor aby uniknac wyciekow pamieci w tescie
        m2_vfs_free_buffer(restored_data.0, restored_data.1);
    }
}

#[test]
fn test_m2_vfs_free_buffer_with_valid_data() {
    let vec = vec![10, 20, 30];
    let buffer = MappedBuffer::new(vec);
    let (ptr, size) = buffer.into_raw();
    
    // Test zwalniania poprawnego bufora
    m2_vfs_free_buffer(ptr, size);
    
    // Nie da sie latwo zbadac uzycia pamieci po zwolnieniu (use after free), 
    // ale samo wykonanie bez bledu/paniki jest oczekiwane.
}

#[test]
fn test_m2_vfs_free_buffer_with_null_and_zero() {
    // Sprawdzamy, czy funkcje FFI dzialaja prawidlowo bez paniki
    // przy blednych wejsciach (zabezpieczenie ffi_guard dziala).
    
    m2_vfs_free_buffer(ptr::null_mut(), 0);
    m2_vfs_free_buffer(ptr::null_mut(), 100);
    
    // Przekazanie nie-null wskaznika ale z rozmiarem 0
    let mut dummy: u8 = 0;
    m2_vfs_free_buffer(&mut dummy as *mut u8, 0);
    
    // Jesli test dojdzie tutaj, to znaczy ze nie bylo paniki.
}
