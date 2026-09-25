use m2_client_bridge::vfs::pack_manager::{
    m2_vfs_pack_manager_clear, m2_vfs_pack_manager_get_count,
    m2_vfs_pack_manager_get_pack_name_by_index, m2_vfs_pack_manager_register_pack,
};
use std::ffi::CString;

#[test]
fn test_vfs_pack_manager_all() {
    // ---------------------------------------------------------
    // test_register_and_sort_packs
    // ---------------------------------------------------------
    m2_vfs_pack_manager_clear();

    let name1 = CString::new("pack_a").unwrap();
    let name2 = CString::new("pack_b").unwrap();
    let name3 = CString::new("pack_c").unwrap();

    assert!(m2_vfs_pack_manager_register_pack(name1.as_ptr(), 10));
    assert!(m2_vfs_pack_manager_register_pack(name2.as_ptr(), 50));
    assert!(m2_vfs_pack_manager_register_pack(name3.as_ptr(), 20));

    assert_eq!(m2_vfs_pack_manager_get_count(), 3);

    let mut buf = vec![0u8; 32];
    
    assert!(m2_vfs_pack_manager_get_pack_name_by_index(0, buf.as_mut_ptr() as *mut _, buf.len()));
    let out = unsafe { std::ffi::CStr::from_ptr(buf.as_ptr() as *const _).to_str().unwrap() };
    assert_eq!(out, "pack_b");

    assert!(m2_vfs_pack_manager_get_pack_name_by_index(1, buf.as_mut_ptr() as *mut _, buf.len()));
    let out = unsafe { std::ffi::CStr::from_ptr(buf.as_ptr() as *const _).to_str().unwrap() };
    assert_eq!(out, "pack_c");

    assert!(m2_vfs_pack_manager_get_pack_name_by_index(2, buf.as_mut_ptr() as *mut _, buf.len()));
    let out = unsafe { std::ffi::CStr::from_ptr(buf.as_ptr() as *const _).to_str().unwrap() };
    assert_eq!(out, "pack_a");

    // ---------------------------------------------------------
    // test_clear_packs
    // ---------------------------------------------------------
    m2_vfs_pack_manager_clear();

    let name = CString::new("test_pack").unwrap();
    m2_vfs_pack_manager_register_pack(name.as_ptr(), 10);
    assert_eq!(m2_vfs_pack_manager_get_count(), 1);

    m2_vfs_pack_manager_clear();
    assert_eq!(m2_vfs_pack_manager_get_count(), 0);

    // ---------------------------------------------------------
    // test_ffi_null_pointers
    // ---------------------------------------------------------
    m2_vfs_pack_manager_clear();

    assert!(!m2_vfs_pack_manager_register_pack(std::ptr::null(), 10));
    assert_eq!(m2_vfs_pack_manager_get_count(), 0);

    let name = CString::new("valid").unwrap();
    m2_vfs_pack_manager_register_pack(name.as_ptr(), 10);

    assert!(!m2_vfs_pack_manager_get_pack_name_by_index(0, std::ptr::null_mut(), 10));

    let mut buf_null = vec![0u8; 10];
    assert!(!m2_vfs_pack_manager_get_pack_name_by_index(0, buf_null.as_mut_ptr() as *mut _, 0));

    // ---------------------------------------------------------
    // test_ffi_bounds
    // ---------------------------------------------------------
    m2_vfs_pack_manager_clear();
    let mut buf_bounds = vec![0u8; 10];
    assert!(!m2_vfs_pack_manager_get_pack_name_by_index(0, buf_bounds.as_mut_ptr() as *mut _, buf_bounds.len()));
}
