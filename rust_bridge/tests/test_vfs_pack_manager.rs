use m2_client_bridge::vfs::pack_manager::{
    m2_vfs_pack_manager_clear, m2_vfs_pack_manager_get_count,
    m2_vfs_pack_manager_register_pack, m2_vfs_pack_manager_exists,
    m2_vfs_pack_manager_load_file, m2_vfs_pack_manager_free_file_content,
};
use std::ffi::CString;

#[test]
fn test_vfs_pack_manager_all() {
    m2_vfs_pack_manager_clear();

    let name1 = CString::new("root").unwrap();
    let dir1 = CString::new("d:/metin2/pack/").unwrap();

    let name2 = CString::new("locale").unwrap();
    let dir2 = CString::new("d:/metin2/pack/").unwrap();

    let name3 = CString::new("uiscript").unwrap();
    let dir3 = CString::new("d:/metin2/pack/").unwrap();

    unsafe {
        assert!(m2_vfs_pack_manager_register_pack(name1.as_ptr(), dir1.as_ptr()));
        assert!(m2_vfs_pack_manager_register_pack(name2.as_ptr(), dir2.as_ptr()));
        assert!(m2_vfs_pack_manager_register_pack(name3.as_ptr(), dir3.as_ptr()));
    }

    assert_eq!(m2_vfs_pack_manager_get_count(), 3);

    // Test file that should exist according to mock
    let exist_path = CString::new("test_file.txt").unwrap();
    let not_exist_path = CString::new("non_existent.txt").unwrap();

    unsafe {
        assert!(m2_vfs_pack_manager_exists(exist_path.as_ptr()));
        assert!(!m2_vfs_pack_manager_exists(not_exist_path.as_ptr()));

        let mut out_size: usize = 0;
        let ptr = m2_vfs_pack_manager_load_file(exist_path.as_ptr(), &mut out_size);
        assert!(!ptr.is_null());
        assert_eq!(out_size, b"test data".len());
        
        // Verify content
        let slice = std::slice::from_raw_parts(ptr, out_size);
        assert_eq!(slice, b"test data");

        // Free the memory
        m2_vfs_pack_manager_free_file_content(ptr, out_size);

        // Load non-existent file
        let ptr_null = m2_vfs_pack_manager_load_file(not_exist_path.as_ptr(), &mut out_size);
        assert!(ptr_null.is_null());
        assert_eq!(out_size, 0);
    }
}
