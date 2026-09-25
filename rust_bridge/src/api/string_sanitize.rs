//! Modul do czyszczenia i normalizacji tekstu (API FFI)

use crate::ffi_guard::safe_ffi_call;
use std::os::raw::c_char;
use std::ffi::{CStr, CString};

pub const FFI_STRING_SANITIZE: u8 = 0;

pub struct StringSanitizer;

impl StringSanitizer {
    pub fn strip_colors(s: &str) -> String {
        let mut result = String::with_capacity(s.len());
        let mut chars = s.chars().peekable();
        let mut in_hyperlink = false;

        while let Some(c) = chars.next() {
            if c == '|' {
                match chars.peek() {
                    Some(&'c') | Some(&'C') => {
                        chars.next();
                        for _ in 0..8 {
                            chars.next();
                        }
                    }
                    Some(&'r') | Some(&'R') => {
                        chars.next();
                    }
                    Some(&'H') => {
                        in_hyperlink = true;
                        chars.next();
                    }
                    Some(&'h') => {
                        in_hyperlink = false;
                        chars.next();
                    }
                    Some(&'|') => {
                        if !in_hyperlink {
                            result.push('|');
                        }
                        chars.next();
                    }
                    _ => {
                        if !in_hyperlink {
                            result.push('|');
                        }
                    }
                }
            } else if !in_hyperlink {
                result.push(c);
            }
        }
        result
    }

    pub fn sanitize_chat(s: &str) -> String {
        s.chars()
            .filter(|&c| {
                let b = c as u32;
                (b >= 32 && b != 127) || b == 9 || b == 10 || b == 13
            })
            .collect()
    }
}

#[no_mangle]
pub unsafe extern "C" fn m2_string_sanitize(src: *const c_char, dst: *mut c_char, max_len: u32) -> bool {
    safe_ffi_call(false, || {
        if src.is_null() || dst.is_null() || max_len == 0 {
            return false;
        }

        let c_str = CStr::from_ptr(src);
        let s = c_str.to_string_lossy();
        
        let sanitized = StringSanitizer::sanitize_chat(&s);
        
        let c_sanitized = match CString::new(sanitized) {
            Ok(cs) => cs,
            Err(_) => return false,
        };

        let bytes = c_sanitized.as_bytes_with_nul();
        if bytes.len() > max_len as usize {
            return false;
        }

        std::ptr::copy_nonoverlapping(bytes.as_ptr(), dst as *mut u8, bytes.len());
        true
    })
}

#[no_mangle]
pub unsafe extern "C" fn m2_strip_color_codes(src: *const c_char) -> *mut c_char {
    safe_ffi_call(std::ptr::null_mut(), || {
        if src.is_null() {
            return std::ptr::null_mut();
        }

        let c_str = CStr::from_ptr(src);
        let s = c_str.to_string_lossy();
        
        let stripped = StringSanitizer::strip_colors(&s);
        
        match CString::new(stripped) {
            Ok(cs) => cs.into_raw(),
            Err(_) => std::ptr::null_mut(),
        }
    })
}

#[no_mangle]
pub unsafe extern "C" fn m2_free_string(ptr: *mut c_char) {
    safe_ffi_call((), || {
        if !ptr.is_null() {
            let _ = CString::from_raw(ptr);
        }
    });
}
