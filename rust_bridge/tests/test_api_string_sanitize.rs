use m2_client_bridge::api::string_sanitize::{
    m2_string_sanitize, m2_string_strip_colors, m2_string_normalize,
};

// Funkcja pomocnicza do testowania
fn call_sanitize(input: &[u8]) -> Vec<u8> {
    let mut dest = vec![0u8; input.len() + 1];
    let written = m2_string_sanitize(
        input.as_ptr(),
        input.len(),
        dest.as_mut_ptr(),
        dest.len(),
    );
    dest.truncate(written);
    dest
}

fn call_strip_colors(input: &[u8]) -> Vec<u8> {
    let mut dest = vec![0u8; input.len() + 1];
    let written = m2_string_strip_colors(
        input.as_ptr(),
        input.len(),
        dest.as_mut_ptr(),
        dest.len(),
    );
    dest.truncate(written);
    dest
}

fn call_normalize(input: &[u8]) -> Vec<u8> {
    let mut dest = vec![0u8; input.len() + 1];
    let written = m2_string_normalize(
        input.as_ptr(),
        input.len(),
        dest.as_mut_ptr(),
        dest.len(),
    );
    dest.truncate(written);
    dest
}

#[test]
fn test_sanitize_basic() {
    let input = b"Hello\x07 World\n";
    let result = call_sanitize(input);
    assert_eq!(result, b"Hello World\n");
}

#[test]
fn test_sanitize_null() {
    let mut dest = vec![0u8; 10];
    let written = m2_string_sanitize(
        std::ptr::null(),
        5,
        dest.as_mut_ptr(),
        dest.len(),
    );
    assert_eq!(written, 0);

    let written = m2_string_sanitize(
        b"test".as_ptr(),
        4,
        std::ptr::null_mut(),
        dest.len(),
    );
    assert_eq!(written, 0);
}

#[test]
fn test_strip_colors() {
    let input = b"To jest |cFFFF0000test|r z |Hitem:123|hlinkiem|h.";
    let result = call_strip_colors(input);
    assert_eq!(result, b"To jest test z linkiem.");
}

#[test]
fn test_strip_colors_escaped() {
    let input = b"Wiadomosc || ukryta.";
    let result = call_strip_colors(input);
    assert_eq!(result, b"Wiadomosc | ukryta.");
}

#[test]
fn test_strip_colors_null() {
    let mut dest = vec![0u8; 10];
    let written = m2_string_strip_colors(
        std::ptr::null(),
        5,
        dest.as_mut_ptr(),
        dest.len(),
    );
    assert_eq!(written, 0);
}

#[test]
fn test_normalize_basic() {
    // a ogonek to 0xB9 w CP1250 / ISO-8859-2
    let input = vec![0xB9, b'b', b'c'];
    let result = call_normalize(&input);
    assert_eq!(result, b"abc");
}

#[test]
fn test_normalize_null() {
    let mut dest = vec![0u8; 10];
    let written = m2_string_normalize(
        std::ptr::null(),
        5,
        dest.as_mut_ptr(),
        dest.len(),
    );
    assert_eq!(written, 0);
}
