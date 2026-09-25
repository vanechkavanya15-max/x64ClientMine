use crate::ffi_guard::safe_ffi_call;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::ptr;
use std::cmp;

/// Struktura przechowujaca liste plikow i pozwalajaca na filtrowanie po rozszerzeniu.
#[derive(Default)]
pub struct DirectoryLister {
    files: Vec<String>,
}

impl DirectoryLister {
    pub fn new() -> Self {
        Self { files: Vec::new() }
    }

    pub fn add_file(&mut self, filename: String) {
        self.files.push(filename);
    }

    pub fn filtered_count(&self, ext: &str) -> usize {
        self.files.iter().filter(|f| f.ends_with(ext)).count()
    }

    pub fn get_filtered_file(&self, ext: &str, index: usize) -> Option<&String> {
        self.files.iter().filter(|f| f.ends_with(ext)).nth(index)
    }
}

// ============================================================================
// C FFI Exports - Directory Lister
// ============================================================================

/// Tworzy nowa instancje DirectoryLister
#[no_mangle]
pub extern "C" fn m2_vfs_dir_lister_new() -> *mut DirectoryLister {
    safe_ffi_call(ptr::null_mut(), || {
        let lister = Box::new(DirectoryLister::new());
        Box::into_raw(lister)
    })
}

/// Zwalnia pamiec po DirectoryLister
#[no_mangle]
pub extern "C" fn m2_vfs_dir_lister_free(ptr: *mut DirectoryLister) {
    safe_ffi_call((), || {
        if !ptr.is_null() {
            unsafe {
                let _ = Box::from_raw(ptr);
            }
        }
    })
}

/// Dodaje nowy plik do listy
#[no_mangle]
pub extern "C" fn m2_vfs_dir_lister_add_file(ptr: *mut DirectoryLister, filename: *const c_char) {
    safe_ffi_call((), || {
        if ptr.is_null() || filename.is_null() {
            return;
        }
        let lister = unsafe { &mut *ptr };
        let c_str = unsafe { CStr::from_ptr(filename) };
        if let Ok(s) = c_str.to_str() {
            lister.add_file(s.to_string());
        }
    })
}

/// Zwraca liczbe plikow z danym rozszerzeniem
#[no_mangle]
pub extern "C" fn m2_vfs_dir_lister_get_filtered_count(ptr: *const DirectoryLister, ext: *const c_char) -> usize {
    safe_ffi_call(0, || {
        if ptr.is_null() || ext.is_null() {
            return 0;
        }
        let lister = unsafe { &*ptr };
        let c_str = unsafe { CStr::from_ptr(ext) };
        if let Ok(s) = c_str.to_str() {
            lister.filtered_count(s)
        } else {
            0
        }
    })
}

/// Kopiuje nazwe pliku pod zadanym indeksem z odfiltrowanej listy do bufora
#[no_mangle]
pub extern "C" fn m2_vfs_dir_lister_get_filtered_file(
    ptr: *const DirectoryLister,
    ext: *const c_char,
    index: usize,
    out_buf: *mut c_char,
    out_buf_len: usize,
) -> bool {
    safe_ffi_call(false, || {
        if ptr.is_null() || ext.is_null() || out_buf.is_null() || out_buf_len == 0 {
            return false;
        }
        let lister = unsafe { &*ptr };
        let c_str = unsafe { CStr::from_ptr(ext) };
        let ext_str = match c_str.to_str() {
            Ok(s) => s,
            Err(_) => return false,
        };

        if let Some(file) = lister.get_filtered_file(ext_str, index) {
            let bytes = file.as_bytes();
            let copy_len = cmp::min(bytes.len(), out_buf_len - 1);
            unsafe {
                ptr::copy_nonoverlapping(bytes.as_ptr(), out_buf as *mut u8, copy_len);
                *out_buf.add(copy_len) = 0; // null terminator
            }
            true
        } else {
            false
        }
    })
}
