use m2_client::chat_buffer_v2::{get_chat_buffer, append_whisper};
use m2_client::py_bridge_chat_minimap::{
    get_minimap_module_state, m2_py_chat_append_chat_v2,
    m2_py_chat_get_line, m2_py_chat_get_line_count, m2_py_minimap_add_waypoint,
    m2_py_minimap_clear_waypoints, m2_py_minimap_hide_minimap, m2_py_minimap_set_minimap_position,
    m2_py_minimap_set_scale, m2_py_minimap_show_minimap,
};
use serial_test::serial;
use std::ffi::CString;

#[test]
#[serial]
fn test_append_chat_v2() {
    get_chat_buffer().lock().unwrap().clear();

    let text = CString::new("Hello World").unwrap();
    let result = m2_py_chat_append_chat_v2(0, text.as_ptr());
    assert!(result, "Expected chat append to succeed");

    let buffer = get_chat_buffer().lock().unwrap();
    assert_eq!(buffer.lines.len(), 1);
    let line = buffer.lines.back().unwrap();
    assert_eq!(line.text, "Hello World");
    assert_eq!(line.sender_name, "");
}

#[test]
#[serial]
fn test_append_whisper() {
    get_chat_buffer().lock().unwrap().clear();

    let result = append_whisper("Jules", "Hi there");
    assert!(result, "Expected whisper append to succeed");

    let buffer = get_chat_buffer().lock().unwrap();
    assert_eq!(buffer.lines.len(), 1);
    let line = buffer.lines.back().unwrap();
    assert_eq!(line.text, "Hi there");
    assert_eq!(line.sender_name, "Jules");
}

#[test]
#[serial]
fn test_get_line_count_and_get_line() {
    get_chat_buffer().lock().unwrap().clear();

    let text = CString::new("First message").unwrap();
    m2_py_chat_append_chat_v2(0, text.as_ptr());

    append_whisper("Alice", "Second message");

    let count = m2_py_chat_get_line_count();
    assert_eq!(count, 2);

    let mut buf = [0i8; 256];

    let success1 = m2_py_chat_get_line(0, buf.as_mut_ptr(), buf.len());
    assert!(success1);
    let line1 = unsafe { std::ffi::CStr::from_ptr(buf.as_ptr()) }.to_str().unwrap();
    assert_eq!(line1, "First message");

    let success2 = m2_py_chat_get_line(1, buf.as_mut_ptr(), buf.len());
    assert!(success2);
    let line2 = unsafe { std::ffi::CStr::from_ptr(buf.as_ptr()) }.to_str().unwrap();
    assert_eq!(line2, "Alice : Second message");
}

#[test]
#[serial]
fn test_minimap_position_scale() {
    let result_pos = m2_py_minimap_set_minimap_position(10.5, 20.5);
    assert!(result_pos);

    let result_scale = m2_py_minimap_set_scale(2.0);
    assert!(result_scale);

    let state = get_minimap_module_state().read().unwrap();
    assert_eq!(state.center_x, 10.5);
    assert_eq!(state.center_y, 20.5);
    assert_eq!(state.scale, 2.0);
}

#[test]
#[serial]
fn test_minimap_waypoints() {
    m2_py_minimap_clear_waypoints();

    let name1 = CString::new("Town").unwrap();
    let result1 = m2_py_minimap_add_waypoint(1.0, 2.0, name1.as_ptr());
    assert!(result1);

    let name2 = CString::new("Forest").unwrap();
    let result2 = m2_py_minimap_add_waypoint(3.0, 4.0, name2.as_ptr());
    assert!(result2);

    {
        let state = get_minimap_module_state().read().unwrap();
        assert_eq!(state.waypoints.len(), 2);
        assert_eq!(state.waypoints[0].x, 1.0);
        assert_eq!(state.waypoints[0].y, 2.0);
        assert_eq!(state.waypoints[0].name, "Town");

        assert_eq!(state.waypoints[1].x, 3.0);
        assert_eq!(state.waypoints[1].y, 4.0);
        assert_eq!(state.waypoints[1].name, "Forest");
    }

    let result_clear = m2_py_minimap_clear_waypoints();
    assert!(result_clear);

    {
        let state = get_minimap_module_state().read().unwrap();
        assert_eq!(state.waypoints.len(), 0);
    }
}

#[test]
#[serial]
fn test_minimap_visibility() {
    let result_hide = m2_py_minimap_hide_minimap();
    assert!(result_hide);

    {
        let state = get_minimap_module_state().read().unwrap();
        assert_eq!(state.is_visible, false);
    }

    let result_show = m2_py_minimap_show_minimap();
    assert!(result_show);

    {
        let state = get_minimap_module_state().read().unwrap();
        assert_eq!(state.is_visible, true);
    }
}