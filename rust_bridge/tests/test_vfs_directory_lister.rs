use m2_client_bridge::vfs::directory_lister::*;
use std::ffi::CString;
use std::os::raw::c_char;

#[test]
fn test_directory_lister_native() {
    let mut lister = DirectoryLister::new();
    
    // Dodajemy pliki testowe
    lister.add_file("effect1.mse".to_string());
    lister.add_file("effect2.mse".to_string());
    lister.add_file("model1.msm".to_string());
    lister.add_file("texture.png".to_string());

    // Sprawdzamy zliczanie po rozszerzeniu
    assert_eq!(lister.filtered_count(".mse"), 2);
    assert_eq!(lister.filtered_count(".msm"), 1);
    assert_eq!(lister.filtered_count(".jpg"), 0);

    // Sprawdzamy pobieranie pliku po indeksie
    assert_eq!(lister.get_filtered_file(".mse", 0).unwrap(), "effect1.mse");
    assert_eq!(lister.get_filtered_file(".mse", 1).unwrap(), "effect2.mse");
    assert!(lister.get_filtered_file(".mse", 2).is_none());
    
    assert_eq!(lister.get_filtered_file(".msm", 0).unwrap(), "model1.msm");
}

#[test]
fn test_directory_lister_ffi() {
    // Tworzenie przez FFI
    let lister_ptr = m2_vfs_dir_lister_new();
    assert!(!lister_ptr.is_null());

    // Dodawanie plikow przez FFI
    let f1 = CString::new("test1.mse").unwrap();
    let f2 = CString::new("test2.mse").unwrap();
    let f3 = CString::new("test.msm").unwrap();

    m2_vfs_dir_lister_add_file(lister_ptr, f1.as_ptr());
    m2_vfs_dir_lister_add_file(lister_ptr, f2.as_ptr());
    m2_vfs_dir_lister_add_file(lister_ptr, f3.as_ptr());

    // Zliczanie przez FFI
    let ext_mse = CString::new(".mse").unwrap();
    let ext_msm = CString::new(".msm").unwrap();
    let ext_jpg = CString::new(".jpg").unwrap();

    let count_mse = m2_vfs_dir_lister_get_filtered_count(lister_ptr, ext_mse.as_ptr());
    assert_eq!(count_mse, 2);

    let count_msm = m2_vfs_dir_lister_get_filtered_count(lister_ptr, ext_msm.as_ptr());
    assert_eq!(count_msm, 1);
    
    let count_jpg = m2_vfs_dir_lister_get_filtered_count(lister_ptr, ext_jpg.as_ptr());
    assert_eq!(count_jpg, 0);

    // Pobieranie przez FFI
    let mut buf = vec![0u8; 256];
    
    let res = m2_vfs_dir_lister_get_filtered_file(
        lister_ptr,
        ext_mse.as_ptr(),
        0,
        buf.as_mut_ptr() as *mut c_char,
        buf.len(),
    );
    assert!(res);
    let out_str = unsafe { std::ffi::CStr::from_ptr(buf.as_ptr() as *const c_char) }
        .to_str()
        .unwrap();
    assert_eq!(out_str, "test1.mse");
    
    let res = m2_vfs_dir_lister_get_filtered_file(
        lister_ptr,
        ext_mse.as_ptr(),
        1,
        buf.as_mut_ptr() as *mut c_char,
        buf.len(),
    );
    assert!(res);
    let out_str = unsafe { std::ffi::CStr::from_ptr(buf.as_ptr() as *const c_char) }
        .to_str()
        .unwrap();
    assert_eq!(out_str, "test2.mse");

    // Proba pobrania spoza zakresu
    let res_out_of_bounds = m2_vfs_dir_lister_get_filtered_file(
        lister_ptr,
        ext_mse.as_ptr(),
        2,
        buf.as_mut_ptr() as *mut c_char,
        buf.len(),
    );
    assert!(!res_out_of_bounds);

    // Zwalnianie pamieci przez FFI
    m2_vfs_dir_lister_free(lister_ptr);
}

#[test]
fn test_directory_lister_ffi_null_pointers() {
    let mut buf = vec![0u8; 256];
    let ext_mse = CString::new(".mse").unwrap();
    let lister_ptr = m2_vfs_dir_lister_new();

    // Dodawanie z null
    m2_vfs_dir_lister_add_file(std::ptr::null_mut(), std::ptr::null());
    m2_vfs_dir_lister_add_file(lister_ptr, std::ptr::null());

    // Zliczanie z null
    assert_eq!(m2_vfs_dir_lister_get_filtered_count(std::ptr::null(), std::ptr::null()), 0);
    assert_eq!(m2_vfs_dir_lister_get_filtered_count(lister_ptr, std::ptr::null()), 0);
    assert_eq!(m2_vfs_dir_lister_get_filtered_count(std::ptr::null(), ext_mse.as_ptr()), 0);

    // Pobieranie z null
    assert!(!m2_vfs_dir_lister_get_filtered_file(
        std::ptr::null(),
        std::ptr::null(),
        0,
        std::ptr::null_mut(),
        0,
    ));

    assert!(!m2_vfs_dir_lister_get_filtered_file(
        lister_ptr,
        ext_mse.as_ptr(),
        0,
        buf.as_mut_ptr() as *mut c_char,
        0, // out_buf_len == 0
    ));

    // Zwalnianie z null
    m2_vfs_dir_lister_free(std::ptr::null_mut());
    
    // Na koniec zwalniamy wlasciwy wskaznik
    m2_vfs_dir_lister_free(lister_ptr);
}
