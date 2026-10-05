use m2_client::py_bridge_chat_filter_sets::*;
use m2_client::chat_buffer_v2::{
    CHAT_TYPE_GUILD, CHAT_TYPE_INFO, CHAT_TYPE_NOTICE, CHAT_TYPE_PARTY, CHAT_TYPE_SHOUT,
    CHAT_TYPE_TALKING,
};

#[test]
fn test_active_chat_set() {
    let id1 = 100;
    let id2 = 200;
    
    m2_py_chat_create_chat_set(id1);
    m2_py_chat_create_chat_set(id2);
    
    m2_py_chat_set_chat_set(id1);
    assert_eq!(*get_active_chat_set().read().unwrap(), id1);
    
    m2_py_chat_set_chat_set(id2);
    assert_eq!(*get_active_chat_set().read().unwrap(), id2);
}

#[test]
fn test_explicit_board_state_position_height() {
    let id = 999;
    m2_py_chat_create_chat_set(id);

    m2_py_chat_set_board_state(id, BOARD_STATE_LOG);
    m2_py_chat_set_position(id, 50, 60);
    m2_py_chat_set_height(id, 120);

    let map = get_chat_set_map().read().unwrap();
    let chat_set = map.get(&id).unwrap();

    assert_eq!(chat_set.board_state, BOARD_STATE_LOG);
    assert_eq!(chat_set.ix, 50);
    assert_eq!(chat_set.iy, 60);
    assert_eq!(chat_set.height, 120);
}

#[test]
fn test_chat_set_creation_and_modification() {
    let id = 1;
    let created_id = m2_py_chat_create_chat_set(id);
    assert_eq!(created_id, id);

    let map = get_chat_set_map().read().unwrap();
    assert!(map.contains_key(&id));
    
    let chat_set = map.get(&id).unwrap();
    assert_eq!(chat_set.board_state, BOARD_STATE_VIEW);
    assert_eq!(chat_set.ix, 0);
    assert_eq!(chat_set.iy, 0);
    assert_eq!(chat_set.width, 0);
    assert_eq!(chat_set.height, 0);
    assert_eq!(chat_set.step, 15);
    assert_eq!(chat_set.end_pos, 1.0);
    assert_eq!(chat_set.align, 0);
}

#[test]
fn test_chat_set_state_updates() {
    let id = 2;
    m2_py_chat_create_chat_set(id);

    m2_py_chat_set_board_state(id, BOARD_STATE_EDIT);
    m2_py_chat_set_position(id, 100, 200);
    m2_py_chat_set_width(id, 300);
    m2_py_chat_set_height(id, 400);
    m2_py_chat_set_step(id, 20);
    m2_py_chat_set_align(id, 1);
    m2_py_chat_set_end_pos(id, 0.5);

    let map = get_chat_set_map().read().unwrap();
    let chat_set = map.get(&id).unwrap();

    assert_eq!(chat_set.board_state, BOARD_STATE_EDIT);
    assert_eq!(chat_set.ix, 100);
    assert_eq!(chat_set.iy, 200);
    assert_eq!(chat_set.width, 300);
    assert_eq!(chat_set.height, 400);
    assert_eq!(chat_set.step, 20);
    assert_eq!(chat_set.align, 1);
    assert_eq!(chat_set.end_pos, 0.5);
}

#[test]
fn test_chat_mode_toggling() {
    let id = 3;
    m2_py_chat_create_chat_set(id);

    // Initial state: all true
    {
        let map = get_chat_set_map().read().unwrap();
        let chat_set = map.get(&id).unwrap();
        assert!(chat_set.mode[CHAT_TYPE_TALKING as usize]);
    }

    // Disable talking
    m2_py_chat_disable_chat_mode(id, CHAT_TYPE_TALKING);
    {
        let map = get_chat_set_map().read().unwrap();
        let chat_set = map.get(&id).unwrap();
        assert!(!chat_set.mode[CHAT_TYPE_TALKING as usize]);
    }

    // Enable talking
    m2_py_chat_enable_chat_mode(id, CHAT_TYPE_TALKING);
    {
        let map = get_chat_set_map().read().unwrap();
        let chat_set = map.get(&id).unwrap();
        assert!(chat_set.mode[CHAT_TYPE_TALKING as usize]);
    }

    // Toggle info (true -> false)
    m2_py_chat_toggle_chat_mode(id, CHAT_TYPE_INFO);
    {
        let map = get_chat_set_map().read().unwrap();
        let chat_set = map.get(&id).unwrap();
        assert!(!chat_set.mode[CHAT_TYPE_INFO as usize]);
    }

    // Toggle info again (false -> true)
    m2_py_chat_toggle_chat_mode(id, CHAT_TYPE_INFO);
    {
        let map = get_chat_set_map().read().unwrap();
        let chat_set = map.get(&id).unwrap();
        assert!(chat_set.mode[CHAT_TYPE_INFO as usize]);
    }

    // Verify all specified types can be filtered (NOTICE, GUILD, PARTY, SHOUT)
    let chat_types_to_test = [
        CHAT_TYPE_TALKING,
        CHAT_TYPE_INFO,
        CHAT_TYPE_NOTICE,
        CHAT_TYPE_GUILD,
        CHAT_TYPE_PARTY,
        CHAT_TYPE_SHOUT,
    ];

    for &chat_type in &chat_types_to_test {
        // Initially true
        {
            let map = get_chat_set_map().read().unwrap();
            let chat_set = map.get(&id).unwrap();
            assert!(chat_set.mode[chat_type as usize]);
        }

        // Disable
        m2_py_chat_disable_chat_mode(id, chat_type);
        {
            let map = get_chat_set_map().read().unwrap();
            let chat_set = map.get(&id).unwrap();
            assert!(!chat_set.mode[chat_type as usize]);
        }

        // Enable
        m2_py_chat_enable_chat_mode(id, chat_type);
        {
            let map = get_chat_set_map().read().unwrap();
            let chat_set = map.get(&id).unwrap();
            assert!(chat_set.mode[chat_type as usize]);
        }
    }
}
