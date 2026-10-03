use serial_test::serial;

use m2_client::py_bridge_app_cursor::*;
use std::ptr;

#[test]
#[serial]
fn test_set_cursor() {
    let mut state = PY_APP_CURSOR.lock().unwrap();
    // Reset state
    *state = AppCursorState::default();
    drop(state);

    // Initial cursor should be Normal
    assert!(m2_py_app_set_cursor(CursorShape::Normal as i32));
    
    let state = PY_APP_CURSOR.lock().unwrap();
    assert_eq!(state.cursor_num, CursorShape::Normal as i32);
    assert_eq!(state.continuous_cursor_num, CursorShape::Normal as i32);
    drop(state);

    // Set continuous cursor (Attack)
    assert!(m2_py_app_set_cursor(CursorShape::Attack as i32));
    
    let state = PY_APP_CURSOR.lock().unwrap();
    assert_eq!(state.cursor_num, CursorShape::Attack as i32);
    // Setting attack doesn't immediately update continuous_cursor_num in C++ logic until a new non-continuous is set or normal is selected
    drop(state);
    
    // Set non-continuous cursor (Door)
    assert!(m2_py_app_set_cursor(CursorShape::Door as i32));
    
    let state = PY_APP_CURSOR.lock().unwrap();
    assert_eq!(state.cursor_num, CursorShape::Door as i32);
    // Since Attack was continuous, it should be saved
    assert_eq!(state.continuous_cursor_num, CursorShape::Attack as i32);
    drop(state);

    // Set Normal, should revert to continuous (Attack)
    assert!(m2_py_app_set_cursor(CursorShape::Normal as i32));
    
    let state = PY_APP_CURSOR.lock().unwrap();
    assert_eq!(state.cursor_num, CursorShape::Attack as i32);
    drop(state);

    // Test out of bounds
    assert!(!m2_py_app_set_cursor(999));
    assert!(!m2_py_app_set_cursor(-1));
}

#[test]
#[serial]
fn test_show_hide_cursor() {
    let mut state = PY_APP_CURSOR.lock().unwrap();
    *state = AppCursorState::default();
    drop(state);

    m2_py_app_hide_cursor();
    
    let state = PY_APP_CURSOR.lock().unwrap();
    assert_eq!(state.cursor_visible, false);
    drop(state);

    m2_py_app_show_cursor();
    
    let state = PY_APP_CURSOR.lock().unwrap();
    assert_eq!(state.cursor_visible, true);
    drop(state);
}

#[test]
#[serial]
fn test_hardware_software_mode() {
    let mut state = PY_APP_CURSOR.lock().unwrap();
    *state = AppCursorState::default();
    drop(state);

    m2_py_app_set_software_cursor();
    
    let state = PY_APP_CURSOR.lock().unwrap();
    assert_eq!(state.cursor_mode, CursorMode::Software);
    drop(state);

    m2_py_app_set_hardware_cursor();
    
    let state = PY_APP_CURSOR.lock().unwrap();
    assert_eq!(state.cursor_mode, CursorMode::Hardware);
    drop(state);
}

#[test]
#[serial]
fn test_get_cursor_position() {
    let mut state = PY_APP_CURSOR.lock().unwrap();
    *state = AppCursorState::default();
    state.mouse_x = 150;
    state.mouse_y = 300;
    drop(state);

    let mut lx = 0;
    let mut ly = 0;

    m2_py_app_get_cursor_position(&mut lx, &mut ly);

    assert_eq!(lx, 150);
    assert_eq!(ly, 300);

    // Test with null pointers to ensure no crash
    m2_py_app_get_cursor_position(ptr::null_mut(), ptr::null_mut());
    m2_py_app_get_cursor_position(&mut lx, ptr::null_mut());
    m2_py_app_get_cursor_position(ptr::null_mut(), &mut ly);
}
