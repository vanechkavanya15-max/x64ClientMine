use std::ffi::CStr;
use std::os::raw::c_char;
use std::ptr;
use crate::ffi_guard::safe_ffi_call;
use crate::crc32::get_crc32;

pub const PATH_RESOLVER: u32 = 0;

pub struct PathResolver;

impl PathResolver {
    /// Normalizuje sciezke poprzez zamiane ukosnikow z backslashy na normalne slesze
    /// oraz zamiane wszystkich liter na male.
    pub fn normalize(path: &str) -> String {
        let mut normalized = String::with_capacity(path.len());
        
        for c in path.chars() {
            if c == '\\' {
                normalized.push('/');
            } else {
                normalized.push(c.to_ascii_lowercase());
            }
        }
        
        normalized
    }

    /// Oblicza CRC32 dla znormalizowanej sciezki.
    pub fn compute_path_crc(path: &str) -> u32 {
        let normalized = Self::normalize(path);
        get_crc32(normalized.as_bytes())
    }
}

// ============================================================================
// C FFI Exports - VFS Path Resolver
// ============================================================================

/// Normalizuje sciezke podana jako c-string (path) i zapisuje w buforze wyjsciowym (out_buf)
/// Zwraca bool (true w przypadku sukcesu, false w przypadku bledu - null pointer, za krotki bufor).
#[no_mangle]
pub unsafe extern "C" fn m2_vfs_normalize_path(
    path: *const c_char,
    out_buf: *mut c_char,
    max_len: u32,
) -> bool {
    safe_ffi_call(false, || {
        if path.is_null() || out_buf.is_null() || max_len == 0 {
            return false;
        }

        let input_cstr = unsafe { CStr::from_ptr(path) };
        let path_str = match input_cstr.to_str() {
            Ok(s) => s,
            Err(_) => return false,
        };

        let normalized = PathResolver::normalize(path_str);
        
        let out_bytes = normalized.as_bytes();
        if out_bytes.len() + 1 > max_len as usize {
            return false; // Bufor jest za maly
        }

        unsafe {
            ptr::copy_nonoverlapping(out_bytes.as_ptr(), out_buf as *mut u8, out_bytes.len());
            *out_buf.add(out_bytes.len()) = 0; // null-terminator
        }

        true
    })
}
