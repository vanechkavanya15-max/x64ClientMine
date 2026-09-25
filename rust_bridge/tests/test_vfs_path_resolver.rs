use m2_client_bridge::vfs::path_resolver::PathResolver;
use m2_client_bridge::crc32::get_crc32;

#[test]
fn test_normalize_path_basic() {
    assert_eq!(PathResolver::normalize("d:/ymir work/ui/intro.sub"), "d:/ymir work/ui/intro.sub");
}

#[test]
fn test_normalize_path_backslash() {
    assert_eq!(PathResolver::normalize("d:\\ymir work\\ui\\intro.sub"), "d:/ymir work/ui/intro.sub");
}

#[test]
fn test_normalize_path_mixed_case() {
    assert_eq!(PathResolver::normalize("D:/YmIr WoRk/Ui/InTrO.sUb"), "d:/ymir work/ui/intro.sub");
}

#[test]
fn test_normalize_path_no_prefix() {
    assert_eq!(PathResolver::normalize("ui/intro.sub"), "ui/intro.sub");
    assert_eq!(PathResolver::normalize("UI\\INTRO.SUB"), "ui/intro.sub");
}

#[test]
fn test_normalize_path_login() {
    assert_eq!(PathResolver::normalize("D:\\Ymir Work\\UI\\login.sub"), "d:/ymir work/ui/login.sub");
}

#[test]
fn test_compute_path_crc() {
    let crc = PathResolver::compute_path_crc("D:\\Ymir Work\\UI\\login.sub");
    assert_eq!(crc, get_crc32(b"d:/ymir work/ui/login.sub"));
}

#[test]
fn test_m2_vfs_normalize_path_ffi() {
    use m2_client_bridge::vfs::path_resolver::m2_vfs_normalize_path;
    use std::ffi::CString;

    let input = CString::new("D:\\ymir work\\UI\\INTRO.sub").unwrap();
    let mut out_buf = vec![0u8; 100];
    
    let res = unsafe { m2_vfs_normalize_path(input.as_ptr(), out_buf.as_mut_ptr() as *mut i8, out_buf.len() as u32) };
    assert_eq!(res, true);
    
    let out_cstr = unsafe { std::ffi::CStr::from_ptr(out_buf.as_ptr() as *const i8) };
    assert_eq!(out_cstr.to_str().unwrap(), "d:/ymir work/ui/intro.sub");
}

#[test]
fn test_m2_vfs_normalize_path_ffi_buffer_too_small() {
    use m2_client_bridge::vfs::path_resolver::m2_vfs_normalize_path;
    use std::ffi::CString;

    let input = CString::new("D:\\ymir work\\UI\\INTRO.sub").unwrap();
    let mut out_buf = vec![0u8; 5]; // Za maly na "d:/ymir work/ui/intro.sub" + '\0'
    
    let res = unsafe { m2_vfs_normalize_path(input.as_ptr(), out_buf.as_mut_ptr() as *mut i8, out_buf.len() as u32) };
    assert_eq!(res, false);
}

#[test]
fn test_m2_vfs_normalize_path_ffi_null_pointers() {
    use m2_client_bridge::vfs::path_resolver::m2_vfs_normalize_path;
    use std::ptr;

    let res = unsafe { m2_vfs_normalize_path(ptr::null(), ptr::null_mut(), 100) };
    assert_eq!(res, false);
}
