use std::ffi::CStr;
use std::os::raw::c_char;
use std::ptr;
use crate::ffi_guard::safe_ffi_call;

/// Normalizuje sciezke poprzez zamiane ukosnikow z backslashy na normalne slesze
/// oraz zamiane wszystkich liter na male, a takze usuniecie prefiksu "d:/ymir work/".
pub fn normalize_path(path: &str) -> String {
    let mut normalized = String::with_capacity(path.len());
    
    for c in path.chars() {
        if c == '\\' {
            normalized.push('/');
        } else {
            normalized.push(c.to_ascii_lowercase());
        }
    }
    
    let prefix = "d:/ymir work/";
    if normalized.starts_with(prefix) {
        normalized.drain(0..prefix.len());
    }
    
    normalized
}

// ============================================================================
// C FFI Exports - VFS Path Resolver
// ============================================================================

/// Normalizuje sciezke podana jako c-string (input) i zapisuje w buforze wyjsciowym (out_buf)
/// Zwraca u32 - 1 w przypadku sukcesu, 0 w przypadku bledu (null pointer, za krotki bufor).
#[no_mangle]
pub extern "C" fn m2_vfs_normalize_path(
    input: *const c_char,
    out_buf: *mut c_char,
    out_len: usize,
) -> u32 {
    safe_ffi_call(0, || {
        if input.is_null() || out_buf.is_null() || out_len == 0 {
            return 0;
        }

        let input_cstr = unsafe { CStr::from_ptr(input) };
        let path_str = match input_cstr.to_str() {
            Ok(s) => s,
            Err(_) => return 0,
        };

        let normalized = normalize_path(path_str);
        
        let out_bytes = normalized.as_bytes();
        if out_bytes.len() + 1 > out_len {
            return 0; // Bufor jest za maly
        }

        unsafe {
            ptr::copy_nonoverlapping(out_bytes.as_ptr(), out_buf as *mut u8, out_bytes.len());
            *out_buf.add(out_bytes.len()) = 0; // null-terminator
        }

        1
    })
}
