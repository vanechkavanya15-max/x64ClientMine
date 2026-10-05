use std::ffi::CStr;
use std::os::raw::c_char;

use m2_client::py_bridge_player_affects::{
    m2_py_player_get_auto_potion_info, m2_py_player_set_auto_potion_info,
    m2_py_player_get_party_member_affects, m2_py_player_set_party_member_affects,
    m2_py_player_get_exchange_item_attribute, m2_py_player_get_exchange_item_metin_socket,
};

use m2_client::recv_exchange_logic_v2::{get_exchange_logic, ExchangeState};

#[test]
fn test_auto_potion_info() {
    let potion_type = 1;
    let activated = true;
    let current = 500;
    let total = 1000;
    let slot = 5;

    // Default shouldn't crash
    let mut out_act = false;
    let mut out_cur = 0;
    let mut out_tot = 0;
    let mut out_slot = 0;
    
    // Set potion info
    assert!(m2_py_player_set_auto_potion_info(potion_type, activated, current, total, slot));
    
    // Get potion info
    assert!(m2_py_player_get_auto_potion_info(
        potion_type,
        &mut out_act,
        &mut out_cur,
        &mut out_tot,
        &mut out_slot,
    ));

    assert_eq!(out_act, activated);
    assert_eq!(out_cur, current);
    assert_eq!(out_tot, total);
    assert_eq!(out_slot, slot);
    
    // Test for nonexistent type
    let mut null_act = true;
    assert!(m2_py_player_get_auto_potion_info(
        999,
        &mut null_act,
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
    ));
    assert_eq!(null_act, false); // Default initialized is false
}

#[test]
fn test_party_member_affects() {
    let vid = 12345;
    
    // Set affects
    assert!(m2_py_player_set_party_member_affects(vid, 10, 20, 30, 40, 50, 60, 70));
    
    // Get affects
    let mut a0 = 0; let mut a1 = 0; let mut a2 = 0; let mut a3 = 0;
    let mut a4 = 0; let mut a5 = 0; let mut a6 = 0;
    
    assert!(m2_py_player_get_party_member_affects(
        vid,
        &mut a0, &mut a1, &mut a2, &mut a3, &mut a4, &mut a5, &mut a6
    ));
    
    assert_eq!(a0, 10);
    assert_eq!(a1, 20);
    assert_eq!(a6, 70);
    
    // Test for nonexistent vid
    assert_eq!(
        m2_py_player_get_party_member_affects(
            99999,
            &mut a0, std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut(),
            std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut(),
        ),
        false
    );
}

#[test]
fn test_exchange_item_inspection() {
    // Modify logic for test
    {
        let mut logic = get_exchange_logic().write().unwrap();
        logic.state = ExchangeState::TradeWindowOpen;
        logic.self_data.item_metin[0][0] = 555;
        logic.self_data.item_attr[0][0].b_type = 12;
        logic.self_data.item_attr[0][0].s_value = 100;
        
        logic.target_data.item_metin[2][1] = 777;
        logic.target_data.item_attr[2][1].b_type = 24;
        logic.target_data.item_attr[2][1].s_value = 200;
    }
    
    // Test self (owner 0) metin socket
    let mut out_socket = 0;
    assert!(m2_py_player_get_exchange_item_metin_socket(0, 0, 0, &mut out_socket));
    assert_eq!(out_socket, 555);
    
    // Test self (owner 0) attribute
    let mut out_type = 0;
    let mut out_val = 0;
    assert!(m2_py_player_get_exchange_item_attribute(0, 0, 0, &mut out_type, &mut out_val));
    assert_eq!(out_type, 12);
    assert_eq!(out_val, 100);
    
    // Test target (owner 1) metin socket
    assert!(m2_py_player_get_exchange_item_metin_socket(1, 2, 1, &mut out_socket));
    assert_eq!(out_socket, 777);
    
    // Test target (owner 1) attribute
    assert!(m2_py_player_get_exchange_item_attribute(1, 2, 1, &mut out_type, &mut out_val));
    assert_eq!(out_type, 24);
    assert_eq!(out_val, 200);
    
    // Test out of bounds pos
    assert_eq!(m2_py_player_get_exchange_item_metin_socket(0, 99, 0, &mut out_socket), false);
    // Test out of bounds idx
    assert_eq!(m2_py_player_get_exchange_item_attribute(0, 0, 99, &mut out_type, &mut out_val), false);
}
