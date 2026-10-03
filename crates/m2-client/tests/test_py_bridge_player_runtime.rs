use m2_client::py_bridge_player_runtime::*;
use m2_client_bridge::state::game_state::{Actor, ItemData, GLOBAL_GAME_STATE, POINT_HP};
use m2_client::recv_quickslot_sync_v2::clear_quickslot_registry;
use serial_test::serial;

#[test]
#[serial]
fn test_player_get_status() {
    GLOBAL_GAME_STATE.write().unwrap().reset();
    GLOBAL_GAME_STATE.write().unwrap().set_point(0, 1337);
    
    unsafe {
        let result = m2_py_player_runtime_GetStatus(std::ptr::null_mut(), std::ptr::null_mut());
        assert_eq!(result as i64, 1337);
    }
}

#[test]
#[serial]
fn test_player_get_item_index() {
    GLOBAL_GAME_STATE.write().unwrap().reset();
    let item = ItemData::new(5001, 1);
    GLOBAL_GAME_STATE.write().unwrap().set_item(1, 0, item);

    unsafe {
        let result = m2_py_player_runtime_GetItemIndex(std::ptr::null_mut(), std::ptr::null_mut());
        assert_eq!(result as i64, 5001);
    }
}

#[test]
#[serial]
fn test_player_get_item_count() {
    GLOBAL_GAME_STATE.write().unwrap().reset();
    let item = ItemData::new(5001, 42);
    GLOBAL_GAME_STATE.write().unwrap().set_item(1, 0, item);

    unsafe {
        let result = m2_py_player_runtime_GetItemCount(std::ptr::null_mut(), std::ptr::null_mut());
        assert_eq!(result as i64, 42);
    }
}

#[test]
#[serial]
fn test_player_get_item_flags() {
    GLOBAL_GAME_STATE.write().unwrap().reset();
    let mut item = ItemData::new(5001, 1);
    item.flags = 99;
    GLOBAL_GAME_STATE.write().unwrap().set_item(1, 0, item);

    unsafe {
        let result = m2_py_player_runtime_GetItemFlags(std::ptr::null_mut(), std::ptr::null_mut());
        assert_eq!(result as i64, 99);
    }
}

#[test]
#[serial]
fn test_player_get_item_metin_socket() {
    GLOBAL_GAME_STATE.write().unwrap().reset();
    let mut item = ItemData::new(5001, 1);
    item.sockets[0] = 777;
    GLOBAL_GAME_STATE.write().unwrap().set_item(1, 0, item);

    unsafe {
        let result = m2_py_player_runtime_GetItemMetinSocket(std::ptr::null_mut(), std::ptr::null_mut());
        assert_eq!(result as i64, 777);
    }
}

#[test]
#[serial]
fn test_player_get_item_attribute() {
    GLOBAL_GAME_STATE.write().unwrap().reset();
    let mut item = ItemData::new(5001, 1);
    item.attrs[0] = (15, 2000);
    GLOBAL_GAME_STATE.write().unwrap().set_item(1, 0, item);

    unsafe {
        let result = m2_py_player_runtime_GetItemAttribute(std::ptr::null_mut(), std::ptr::null_mut());
        assert_eq!(result as i64, 15);
    }
}

#[test]
#[serial]
fn test_player_get_main_character_info() {
    GLOBAL_GAME_STATE.write().unwrap().reset();
    GLOBAL_GAME_STATE.write().unwrap().set_player_vid(100);
    GLOBAL_GAME_STATE.write().unwrap().set_player_pos(10.0, 20.0, 30.0, 45.0);
    let mut actor = Actor::default();
    actor.vid = 100;
    actor.name = "Hero".to_string();
    GLOBAL_GAME_STATE.write().unwrap().add_or_update_actor(actor);

    unsafe {
        let result = m2_py_player_runtime_GetName(std::ptr::null_mut(), std::ptr::null_mut());
        assert_eq!(result as i64, 1);

        let result_idx = m2_py_player_runtime_GetMainCharacterIndex(std::ptr::null_mut(), std::ptr::null_mut());
        assert_eq!(result_idx as i64, 100);
        
        let result_pos = m2_py_player_runtime_GetMainCharacterPosition(std::ptr::null_mut(), std::ptr::null_mut());
        assert_eq!(result_pos as i64, 1);
    }
}

#[test]
#[serial]
fn test_player_quickslot_sync() {
    clear_quickslot_registry();
    
    unsafe {
        let result_set = m2_py_player_runtime_SetQuickSlot(std::ptr::null_mut(), std::ptr::null_mut());
        assert_eq!(result_set as i64, 1);

        let result_get = m2_py_player_runtime_GetQuickSlot(std::ptr::null_mut(), std::ptr::null_mut());
        assert_eq!(result_get as i64, 0); 
    }
}
