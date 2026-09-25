use m2_client_terrain::terrain_ffi::{
    m2_terrain_load, m2_terrain_get_height, m2_terrain_is_blocked, m2_terrain_free
};
use std::ffi::CString;

#[test]
fn test_terrain_load_valid_path() {
    let path = CString::new("test_path").unwrap();
    let terrain = m2_terrain_load(path.as_ptr());
    assert!(!terrain.is_null());
    
    // Zwalniamy pamiec
    m2_terrain_free(terrain);
}

#[test]
fn test_terrain_load_null_path() {
    let terrain = m2_terrain_load(std::ptr::null());
    assert!(terrain.is_null());
}

#[test]
fn test_terrain_get_height_valid() {
    let path = CString::new("test_path").unwrap();
    let terrain = m2_terrain_load(path.as_ptr());
    assert!(!terrain.is_null());
    
    let height = m2_terrain_get_height(terrain, 10.0, 20.0);
    assert_eq!(height, 0.0);
    
    m2_terrain_free(terrain);
}

#[test]
fn test_terrain_get_height_null() {
    let height = m2_terrain_get_height(std::ptr::null_mut(), 10.0, 20.0);
    assert_eq!(height, 0.0);
}

#[test]
fn test_terrain_is_blocked_valid() {
    let path = CString::new("test_path").unwrap();
    let terrain = m2_terrain_load(path.as_ptr());
    assert!(!terrain.is_null());
    
    let blocked = m2_terrain_is_blocked(terrain, 10.0, 20.0);
    assert_eq!(blocked, false);
    
    m2_terrain_free(terrain);
}

#[test]
fn test_terrain_is_blocked_null() {
    let blocked = m2_terrain_is_blocked(std::ptr::null_mut(), 10.0, 20.0);
    assert_eq!(blocked, true);
}

#[test]
fn test_terrain_free_null() {
    // Nie powinno byc paniki
    m2_terrain_free(std::ptr::null_mut());
}
