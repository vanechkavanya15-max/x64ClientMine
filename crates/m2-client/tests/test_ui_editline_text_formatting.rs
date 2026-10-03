// test_ui_editline_text_formatting.rs - Tests for EditLine Text Formatting Ext in Metin2 Pure Rust

#[path = "../src/ui_editline_text_formatting.rs"]
mod ui_editline_text_formatting;

use ui_editline_text_formatting::{
    wndMgr_SetLimitWidth, wndMgr_SetMultiLine, wndMgr_SetSecret,
    wndMgr_SetMax, wndMgr_SetOutline, wndMgr_SetFeather, wndMgr_SetFontName,
    TEXT_FORMAT_STATES,
};
use std::ffi::CString;

#[test]
fn test_wndmgr_set_limit_width() {
    let handle = 1001;
    wndMgr_SetLimitWidth(handle, 250.0);
    
    let map = TEXT_FORMAT_STATES.read().unwrap();
    let state = map.get(&handle).unwrap();
    assert_eq!(state.limit_width, 250.0);
}

#[test]
fn test_wndmgr_set_multi_line() {
    let handle = 1002;
    wndMgr_SetMultiLine(handle, true);
    
    let map = TEXT_FORMAT_STATES.read().unwrap();
    let state = map.get(&handle).unwrap();
    assert!(state.multi_line);
}

#[test]
fn test_wndmgr_set_secret() {
    let handle = 1003;
    wndMgr_SetSecret(handle, true);
    
    let map = TEXT_FORMAT_STATES.read().unwrap();
    let state = map.get(&handle).unwrap();
    assert!(state.is_secret);
}

#[test]
fn test_wndmgr_set_max() {
    let handle = 1004;
    wndMgr_SetMax(handle, 64);
    
    let map = TEXT_FORMAT_STATES.read().unwrap();
    let state = map.get(&handle).unwrap();
    assert_eq!(state.max_chars, 64);
}

#[test]
fn test_wndmgr_set_outline_and_feather() {
    let handle = 1005;
    wndMgr_SetOutline(handle, true);
    wndMgr_SetFeather(handle, true);
    
    let map = TEXT_FORMAT_STATES.read().unwrap();
    let state = map.get(&handle).unwrap();
    assert!(state.is_outline);
    assert!(state.is_feather);
}

#[test]
fn test_wndmgr_set_font_name() {
    let handle = 1006;
    let font_name = CString::new("Tahoma:12").unwrap();
    
    wndMgr_SetFontName(handle, font_name.as_ptr());
    
    let map = TEXT_FORMAT_STATES.read().unwrap();
    let state = map.get(&handle).unwrap();
    assert_eq!(state.font_name, "Tahoma:12");
}

#[test]
fn test_wndmgr_state_defaults() {
    let handle = 1007;
    // Nie ustalamy nic
    wndMgr_SetMax(handle, 10);
    
    let map = TEXT_FORMAT_STATES.read().unwrap();
    let state = map.get(&handle).unwrap();
    assert_eq!(state.limit_width, 0.0);
    assert!(!state.multi_line);
    assert!(!state.is_secret);
    assert!(!state.is_outline);
    assert!(!state.is_feather);
    assert!(state.font_name.is_empty());
}
