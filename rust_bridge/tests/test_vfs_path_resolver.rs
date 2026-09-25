use m2_client_bridge::vfs::path_resolver::normalize_path;

#[test]
fn test_normalize_path_basic() {
    assert_eq!(normalize_path("d:/ymir work/ui/intro.sub"), "ui/intro.sub");
}

#[test]
fn test_normalize_path_backslash() {
    assert_eq!(normalize_path("d:\\ymir work\\ui\\intro.sub"), "ui/intro.sub");
}

#[test]
fn test_normalize_path_mixed_case() {
    assert_eq!(normalize_path("D:/YmIr WoRk/Ui/InTrO.sUb"), "ui/intro.sub");
}

#[test]
fn test_normalize_path_no_prefix() {
    assert_eq!(normalize_path("ui/intro.sub"), "ui/intro.sub");
    assert_eq!(normalize_path("UI\\INTRO.SUB"), "ui/intro.sub");
}

#[test]
fn test_m2_vfs_normalize_path_ffi() {
    use m2_client_bridge::vfs::path_resolver::m2_vfs_normalize_path;
    use std::ffi::CString;

    let input = CString::new("D:\\ymir work\\UI\\INTRO.sub").unwrap();
    let mut out_buf = vec![0u8; 100];
    
    let res = m2_vfs_normalize_path(input.as_ptr(), out_buf.as_mut_ptr() as *mut i8, out_buf.len());
    assert_eq!(res, 1);
    
    let out_cstr = unsafe { std::ffi::CStr::from_ptr(out_buf.as_ptr() as *const i8) };
    assert_eq!(out_cstr.to_str().unwrap(), "ui/intro.sub");
}

#[test]
fn test_m2_vfs_normalize_path_ffi_buffer_too_small() {
    use m2_client_bridge::vfs::path_resolver::m2_vfs_normalize_path;
    use std::ffi::CString;

    let input = CString::new("D:\\ymir work\\UI\\INTRO.sub").unwrap();
    let mut out_buf = vec![0u8; 5]; // Za maly na "ui/intro.sub" + '\0'
    
    let res = m2_vfs_normalize_path(input.as_ptr(), out_buf.as_mut_ptr() as *mut i8, out_buf.len());
    assert_eq!(res, 0);
}

#[test]
fn test_m2_vfs_normalize_path_ffi_null_pointers() {
    use m2_client_bridge::vfs::path_resolver::m2_vfs_normalize_path;
    use std::ptr;

    let res = m2_vfs_normalize_path(ptr::null(), ptr::null_mut(), 100);
    assert_eq!(res, 0);
}
