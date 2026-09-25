use crate::ffi_guard::safe_ffi_call;
use std::os::raw::c_char;
use std::ffi::CStr;
use std::ptr;

/// Sprawdza czy plik istnieje w systemie plikow.
/// Zwraca true jesli plik istnieje, false w przeciwnym wypadku.
#[no_mangle]
pub extern "C" fn m2_vfs_exists(path: *const c_char) -> bool {
    safe_ffi_call(false, || {
        if path.is_null() {
            return false;
        }
        let c_str = unsafe { CStr::from_ptr(path) };
        if let Ok(path_str) = c_str.to_str() {
            std::path::Path::new(path_str).exists()
        } else {
            false
        }
    })
}

/// Laduje plik do pamieci z systemu plikow i zwraca wskaznik do zaalokowanego bufora.
/// Ustawia out_size na rozmiar pliku.
/// Zwrocony bufor musi byc zwolniony za pomoca m2_vfs_free.
#[no_mangle]
pub extern "C" fn m2_vfs_load_file(path: *const c_char, out_size: *mut usize) -> *mut u8 {
    safe_ffi_call(ptr::null_mut(), || {
        if path.is_null() || out_size.is_null() {
            return ptr::null_mut();
        }
        
        let c_str = unsafe { CStr::from_ptr(path) };
        let path_str = match c_str.to_str() {
            Ok(s) => s,
            Err(_) => return ptr::null_mut(),
        };
        
        match std::fs::read(path_str) {
            Ok(data) => {
                let mut boxed_slice = data.into_boxed_slice();
                unsafe { *out_size = boxed_slice.len() };
                let ptr = boxed_slice.as_mut_ptr();
                std::mem::forget(boxed_slice);
                ptr
            }
            Err(_) => {
                unsafe { *out_size = 0 };
                ptr::null_mut()
            }
        }
    })
}

/// Zwalnia pamiec zaalokowana przez m2_vfs_load_file.
#[no_mangle]
pub extern "C" fn m2_vfs_free(data: *mut u8, size: usize) {
    safe_ffi_call((), || {
        if data.is_null() {
            return;
        }
        unsafe {
            let slice = ptr::slice_from_raw_parts_mut(data, size);
            let _ = Box::from_raw(slice);
        }
    });
}
