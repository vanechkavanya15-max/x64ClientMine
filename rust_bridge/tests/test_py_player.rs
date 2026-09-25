use m2_client_bridge::api::py_player::{
    m2_py_player_get_item_count, m2_py_player_get_item_index, m2_py_player_get_position,
    m2_py_player_get_status,
};

#[test]
fn test_py_player_status_ffi() {
    assert_eq!(m2_py_player_get_status(1), 1000);
    assert_eq!(m2_py_player_get_status(5), 1000);
    assert_eq!(m2_py_player_get_item_index(1, 0), 0);
    assert_eq!(m2_py_player_get_item_count(1, 0), 0);
}

#[test]
fn test_py_player_position_ffi() {
    let mut x = 999;
    let mut y = 999;
    let mut z = 999;
    unsafe {
        assert!(m2_py_player_get_position(&mut x, &mut y, &mut z));
        assert_eq!(x, 0);
        assert!(!m2_py_player_get_position(std::ptr::null_mut(), &mut y, &mut z));
    }
}
