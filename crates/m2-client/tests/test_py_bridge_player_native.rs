use m2_client::py_bridge_player_native::*;
use m2_client_bridge::state::game_state::{Actor, ItemData, GLOBAL_GAME_STATE, POINT_GOLD, POINT_HP};
use serial_test::serial;

#[test]
#[serial]
fn test_player_get_status() {
    GLOBAL_GAME_STATE.write().unwrap().reset();
    GLOBAL_GAME_STATE.write().unwrap().set_point(POINT_HP, 1337);
    
    unsafe {
        let result = m2_py_player_GetStatus(std::ptr::null_mut(), std::ptr::null_mut());
        assert_eq!(result as i64, 0);
        
        GLOBAL_GAME_STATE.write().unwrap().set_point(0, 999);
        let result2 = m2_py_player_GetStatus(std::ptr::null_mut(), std::ptr::null_mut());
        assert_eq!(result2 as i64, 999);
    }
}

#[test]
#[serial]
fn test_player_get_item_index() {
    GLOBAL_GAME_STATE.write().unwrap().reset();
    let item = ItemData::new(5001, 1);
    GLOBAL_GAME_STATE.write().unwrap().set_item(0, 0, item);

    unsafe {
        let result = m2_py_player_GetItemIndex(std::ptr::null_mut(), std::ptr::null_mut());
        assert_eq!(result as i64, 5001);
    }
}

#[test]
#[serial]
fn test_player_get_item_count() {
    GLOBAL_GAME_STATE.write().unwrap().reset();
    let item = ItemData::new(5001, 42);
    GLOBAL_GAME_STATE.write().unwrap().set_item(0, 0, item);

    unsafe {
        let result = m2_py_player_GetItemCount(std::ptr::null_mut(), std::ptr::null_mut());
        assert_eq!(result as i64, 42);
    }
}

#[test]
#[serial]
fn test_player_get_item_flags() {
    GLOBAL_GAME_STATE.write().unwrap().reset();
    let mut item = ItemData::new(5001, 1);
    item.flags = 99;
    GLOBAL_GAME_STATE.write().unwrap().set_item(0, 0, item);

    unsafe {
        let result = m2_py_player_GetItemFlags(std::ptr::null_mut(), std::ptr::null_mut());
        assert_eq!(result as i64, 99);
    }
}

#[test]
#[serial]
fn test_player_get_item_metin_socket() {
    GLOBAL_GAME_STATE.write().unwrap().reset();
    let mut item = ItemData::new(5001, 1);
    item.sockets[0] = 777;
    GLOBAL_GAME_STATE.write().unwrap().set_item(0, 0, item);

    unsafe {
        let result = m2_py_player_GetItemMetinSocket(std::ptr::null_mut(), std::ptr::null_mut());
        assert_eq!(result as i64, 777);
    }
}

#[test]
#[serial]
fn test_player_get_item_attribute() {
    GLOBAL_GAME_STATE.write().unwrap().reset();
    let mut item = ItemData::new(5001, 1);
    item.attrs[0] = (15, 2000);
    GLOBAL_GAME_STATE.write().unwrap().set_item(0, 0, item);

    unsafe {
        let result = m2_py_player_GetItemAttribute(std::ptr::null_mut(), std::ptr::null_mut());
        assert_eq!(result as i64, 1);
    }
}

#[test]
#[serial]
fn test_player_get_money() {
    GLOBAL_GAME_STATE.write().unwrap().reset();
    GLOBAL_GAME_STATE.write().unwrap().set_point(POINT_GOLD, 100000);

    unsafe {
        let result = m2_py_player_GetMoney(std::ptr::null_mut(), std::ptr::null_mut());
        assert_eq!(result as i64, 100000);
    }
}

#[test]
#[serial]
fn test_player_get_name() {
    GLOBAL_GAME_STATE.write().unwrap().reset();
    GLOBAL_GAME_STATE.write().unwrap().set_player_vid(100);
    let mut actor = Actor::default();
    actor.vid = 100;
    actor.name = "Hero".to_string();
    GLOBAL_GAME_STATE.write().unwrap().add_or_update_actor(actor);

    unsafe {
        let result = m2_py_player_GetName(std::ptr::null_mut(), std::ptr::null_mut());
        assert_eq!(result as i64, 1);
    }
}

#[test]
#[serial]
fn test_player_get_main_character_position() {
    GLOBAL_GAME_STATE.write().unwrap().reset();
    GLOBAL_GAME_STATE.write().unwrap().set_player_pos(10.0, 20.0, 30.0, 45.0);

    unsafe {
        let result = m2_py_player_GetMainCharacterPosition(std::ptr::null_mut(), std::ptr::null_mut());
        assert_eq!(result as i64, 1);
    }
}

#[test]
#[serial]
fn test_player_get_main_character_index() {
    GLOBAL_GAME_STATE.write().unwrap().reset();
    GLOBAL_GAME_STATE.write().unwrap().set_player_vid(12345);

    unsafe {
        let result = m2_py_player_GetMainCharacterIndex(std::ptr::null_mut(), std::ptr::null_mut());
        assert_eq!(result as i64, 12345);
    }
}