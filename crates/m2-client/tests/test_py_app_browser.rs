use m2_client::py_bridge_app_browser_complete::*;
use std::ffi::CString;

#[test]
fn test_app_browser_lifecycle() {
    std::env::set_var("M2_HEADLESS_TEST", "1");

    let valid_url = CString::new("https://metin2.pl/itemshop").unwrap();
    assert!(m2_py_app_open_web_browser(valid_url.as_ptr()));
    assert!(m2_py_app_is_web_browser_open());

    let mut buf = [0u8; 64];
    assert!(m2_py_app_get_last_opened_url(buf.as_mut_ptr() as *mut _, buf.len()));
    let url = std::str::from_utf8(&buf).unwrap().trim_matches('\0');
    assert_eq!(url, "https://metin2.pl/itemshop");

    assert!(m2_py_app_close_web_browser());
    assert!(!m2_py_app_is_web_browser_open());

    // Invalid URL rejection
    let invalid_url = CString::new("javascript:alert(1)").unwrap();
    assert!(!m2_py_app_open_web_browser(invalid_url.as_ptr()));

    // Window flash
    assert!(m2_py_app_flash_window());
}
