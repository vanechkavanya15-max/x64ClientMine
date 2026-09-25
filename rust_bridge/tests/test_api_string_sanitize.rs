use m2_client_bridge::api::string_sanitize::{
    m2_string_sanitize, m2_strip_color_codes, m2_free_string, StringSanitizer,
};
use std::ffi::{CStr, CString};

#[test]
fn test_string_sanitizer_strip_colors() {
    let input = "|cffff0000Tekst|r";
    assert_eq!(StringSanitizer::strip_colors(input), "Tekst");

    let input_link = "To jest |cFFFF0000test|r z |Hitem:123|hlinkiem|h.";
    assert_eq!(StringSanitizer::strip_colors(input_link), "To jest test z linkiem.");

    let input_escaped = "Wiadomosc || ukryta.";
    assert_eq!(StringSanitizer::strip_colors(input_escaped), "Wiadomosc | ukryta.");
}

#[test]
fn test_string_sanitizer_sanitize_chat() {
    let input = "Hello\x07 World\n";
    assert_eq!(StringSanitizer::sanitize_chat(input), "Hello World\n");
}

#[test]
fn test_ffi_m2_string_sanitize() {
    let input = CString::new("Hello\x07 World\n").unwrap();
    let mut dest = vec![0u8; 100];
    
    let result = unsafe {
        m2_string_sanitize(
            input.as_ptr(),
            dest.as_mut_ptr() as *mut std::os::raw::c_char,
            dest.len() as u32,
        )
    };
    
    assert!(result);
    
    let c_str = unsafe { CStr::from_ptr(dest.as_ptr() as *const std::os::raw::c_char) };
    assert_eq!(c_str.to_str().unwrap(), "Hello World\n");
}

#[test]
fn test_ffi_m2_string_sanitize_null() {
    let mut dest = vec![0u8; 10];
    let result = unsafe {
        m2_string_sanitize(
            std::ptr::null(),
            dest.as_mut_ptr() as *mut std::os::raw::c_char,
            dest.len() as u32,
        )
    };
    assert!(!result);

    let input = CString::new("test").unwrap();
    let result = unsafe {
        m2_string_sanitize(
            input.as_ptr(),
            std::ptr::null_mut(),
            10,
        )
    };
    assert!(!result);
}

#[test]
fn test_ffi_m2_strip_color_codes() {
    let input = CString::new("|cffff0000Tekst|r").unwrap();
    
    let result_ptr = unsafe { m2_strip_color_codes(input.as_ptr()) };
    assert!(!result_ptr.is_null());
    
    let c_str = unsafe { CStr::from_ptr(result_ptr) };
    assert_eq!(c_str.to_str().unwrap(), "Tekst");
    
    unsafe { m2_free_string(result_ptr) };
}

#[test]
fn test_ffi_m2_strip_color_codes_null() {
    let result_ptr = unsafe { m2_strip_color_codes(std::ptr::null()) };
    assert!(result_ptr.is_null());
}

#[test]
fn test_ffi_m2_free_string_null() {
    // Should not panic or crash
    unsafe { m2_free_string(std::ptr::null_mut()) };
}
