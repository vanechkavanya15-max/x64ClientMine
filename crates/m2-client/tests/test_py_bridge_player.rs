use m2_client::py_bridge_player::{
    m2_py_player_get_item_count, m2_py_player_get_item_count_by_vnum, m2_py_player_get_item_index,
    m2_py_player_get_main_character_position, m2_py_player_get_status, m2_py_player_pick_close_item,
    m2_py_player_send_click_item, m2_py_player_set_status,
};
use m2_client_bridge::state::game_state::{ItemData, GLOBAL_GAME_STATE};
use serial_test::serial;

#[test]
#[serial]
fn test_py_player_get_status() {
    GLOBAL_GAME_STATE.write().unwrap().reset();
    unsafe {
        m2_py_player_set_status(5, 1234); // POINT_HP = 5
        assert_eq!(m2_py_player_get_status(5), 1234);

        m2_py_player_set_status(11, 99999); // POINT_GOLD = 11
        assert_eq!(m2_py_player_get_status(11), 99999);
    }
}

#[test]
#[serial]
fn test_py_player_inventory() {
    GLOBAL_GAME_STATE.write().unwrap().reset();
    GLOBAL_GAME_STATE
        .write()
        .unwrap()
        .set_item(1, 0, ItemData::new(1001, 15));
    GLOBAL_GAME_STATE
        .write()
        .unwrap()
        .set_item(1, 1, ItemData::new(1001, 5));
    GLOBAL_GAME_STATE
        .write()
        .unwrap()
        .set_item(1, 2, ItemData::new(2002, 10));

    unsafe {
        assert_eq!(m2_py_player_get_item_index(1, 0), 1001);
        assert_eq!(m2_py_player_get_item_count(1, 0), 15);
        assert_eq!(m2_py_player_get_item_index(1, 2), 2002);
        assert_eq!(m2_py_player_get_item_count(1, 2), 10);
        assert_eq!(m2_py_player_get_item_index(2, 0), 0); // Empty window
        
        // Sum counts for vnum 1001
        assert_eq!(m2_py_player_get_item_count_by_vnum(1001), 20);
        assert_eq!(m2_py_player_get_item_count_by_vnum(2002), 10);
        assert_eq!(m2_py_player_get_item_count_by_vnum(9999), 0);
    }
}

#[test]
#[serial]
fn test_py_player_get_position() {
    GLOBAL_GAME_STATE.write().unwrap().reset();
    GLOBAL_GAME_STATE
        .write()
        .unwrap()
        .set_player_pos(100.0, 200.0, 300.0, 45.0);

    let mut out_x = 0.0;
    let mut out_y = 0.0;
    let mut out_z = 0.0;

    unsafe {
        let result = m2_py_player_get_main_character_position(&mut out_x, &mut out_y, &mut out_z);
        assert!(result);
        assert_eq!(out_x, 100.0);
        assert_eq!(out_y, 200.0);
        assert_eq!(out_z, 300.0);

        // Null pointer safety
        assert!(!m2_py_player_get_main_character_position(
            std::ptr::null_mut(),
            &mut out_y,
            &mut out_z
        ));
    }
}

#[test]
#[serial]
fn test_py_player_packets() {
    // Tests FFI safety
    unsafe {
        assert!(m2_py_player_send_click_item(1, 0));
        assert!(m2_py_player_pick_close_item());
    }
}
