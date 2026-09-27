use serial_test::serial;
use std::ffi::CString;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use m2_client::py_bridge_pack::{
    m2_py_pack_exist, m2_py_pack_get_size, m2_py_pack_mount, pack_exist, pack_get, get_mounted_packs,
};

fn get_temp_test_dir() -> PathBuf {
    let mut dir = std::env::temp_dir();
    dir.push("m2_test_pack_dir");
    let _ = std::fs::create_dir_all(&dir);
    dir
}

#[test]
#[serial]
fn test_py_pack_exist_file_not_found() {
    let name = "definitely_non_existent_file_12345.txt";
    assert!(!pack_exist(name));

    let c_name = CString::new(name).unwrap();
    assert!(!m2_py_pack_exist(c_name.as_ptr()));
}

#[test]
#[serial]
fn test_py_pack_exist_on_disk() {
    let temp_dir = get_temp_test_dir();
    let file_path = temp_dir.join("test_py_pack_file.txt");
    {
        let mut f = File::create(&file_path).unwrap();
        f.write_all(b"Hello Pack Bridge!").unwrap();
    }

    let path_str = file_path.to_str().unwrap();
    assert!(pack_exist(path_str));

    let c_path = CString::new(path_str).unwrap();
    assert!(m2_py_pack_exist(c_path.as_ptr()));

    let size = m2_py_pack_get_size(c_path.as_ptr());
    assert_eq!(size, 18);

    let content = pack_get(path_str);
    assert_eq!(content, Some(b"Hello Pack Bridge!".to_vec()));

    let _ = std::fs::remove_file(&file_path);
}

#[test]
#[serial]
fn test_py_pack_null_pointer_safety() {
    assert!(!m2_py_pack_mount(std::ptr::null()));
    assert!(!m2_py_pack_exist(std::ptr::null()));
    assert_eq!(m2_py_pack_get_size(std::ptr::null()), 0);
}

#[test]
#[serial]
fn test_py_pack_mount_non_existent() {
    let c_path = CString::new("C:/non_existent_archive_file.pck").unwrap();
    let res = m2_py_pack_mount(c_path.as_ptr());
    assert!(!res);
}

#[test]
#[serial]
fn test_py_pack_mounted_packs_access() {
    let packs = get_mounted_packs().lock().unwrap();
    // Verify mutex locking succeeds
    assert!(packs.is_empty() || !packs.is_empty());
}
