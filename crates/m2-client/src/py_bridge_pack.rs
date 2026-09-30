use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::sync::{Mutex, OnceLock};
use std::collections::HashMap;

use m2_client_bridge::ffi_guard::safe_ffi_call;
use crate::vfs_pck_memory_mapped::{mount_archive, PckArchive};

static MOUNTED_PACKS: OnceLock<Mutex<HashMap<String, PckArchive>>> = OnceLock::new();

pub fn get_mounted_packs() -> &'static Mutex<HashMap<String, PckArchive>> {
    MOUNTED_PACKS.get_or_init(|| Mutex::new(HashMap::new()))
}

pub fn mount_pack_file(path: &str) -> bool {
    if let Ok(archive) = mount_archive(path) {
        let mut packs = get_mounted_packs().lock().unwrap();
        packs.insert(path.to_string(), archive);
        true
    } else {
        false
    }
}

#[no_mangle]
pub extern "C" fn m2_py_pack_mount(archive_path: *const c_char) -> bool {
    safe_ffi_call(false, || {
        if archive_path.is_null() {
            return false;
        }
        let c_str = unsafe { CStr::from_ptr(archive_path) };
        let path_str = match c_str.to_str() {
            Ok(s) => s,
            Err(_) => return false,
        };

        mount_pack_file(path_str)
    })
}

#[no_mangle]
pub extern "C" fn m2_py_pack_exist(filename: *const c_char) -> bool {
    safe_ffi_call(false, || {
        if filename.is_null() {
            return false;
        }
        let c_str = unsafe { CStr::from_ptr(filename) };
        let name_str = match c_str.to_str() {
            Ok(s) => s,
            Err(_) => return false,
        };

        // Sprawdz najpierw na dysku
        if std::path::Path::new(name_str).exists() {
            return true;
        }

        // Sprawdz w zamontowanych archiwach
        let packs = get_mounted_packs().lock().unwrap();
        for archive in packs.values() {
            if crate::vfs_pck_memory_mapped::read_file_from_archive(archive, name_str).is_some() {
                return true;
            }
        }
        false
    })
}

#[no_mangle]
pub extern "C" fn m2_py_pack_get_size(filename: *const c_char) -> usize {
    safe_ffi_call(0, || {
        if filename.is_null() {
            return 0;
        }
        let c_str = unsafe { CStr::from_ptr(filename) };
        let name_str = match c_str.to_str() {
            Ok(s) => s,
            Err(_) => return 0,
        };

        if let Ok(metadata) = std::fs::metadata(name_str) {
            return metadata.len() as usize;
        }

        let packs = get_mounted_packs().lock().unwrap();
        for archive in packs.values() {
            if let Some(data) = crate::vfs_pck_memory_mapped::read_file_from_archive(archive, name_str) {
                return data.len();
            }
        }
        0
    })
}

#[no_mangle]
pub extern "C" fn m2_py_pack_read_data(filename: *const c_char, out_buf: *mut u8, max_len: usize) -> usize {
    safe_ffi_call(0, || {
        if filename.is_null() || out_buf.is_null() || max_len == 0 {
            return 0;
        }
        let c_str = unsafe { CStr::from_ptr(filename) };
        let name_str = match c_str.to_str() {
            Ok(s) => s,
            Err(_) => return 0,
        };

        if let Some(data) = pack_get(name_str) {
            let copy_len = data.len().min(max_len);
            unsafe {
                std::ptr::copy_nonoverlapping(data.as_ptr(), out_buf, copy_len);
            }
            copy_len
        } else {
            0
        }
    })
}

pub fn pack_exist(filename: &str) -> bool {
    if let Ok(c_str) = CString::new(filename) {
        m2_py_pack_exist(c_str.as_ptr())
    } else {
        false
    }
}

pub fn pack_get(filename: &str) -> Option<Vec<u8>> {
    let packs = get_mounted_packs().lock().unwrap();
    for archive in packs.values() {
        if let Some(data) = crate::vfs_pck_memory_mapped::read_file_from_archive(archive, filename) {
            return Some(data);
        }
    }
    if let Ok(data) = std::fs::read(filename) {
        return Some(data);
    }
    None
}
