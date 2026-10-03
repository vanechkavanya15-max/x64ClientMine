use m2_client::py_bridge_item_details::*;
use m2_client::item_proto_binary_loader::{
    TItemTable, TItemApply, TItemLimit, ITEM_NAME_MAX_LEN, ITEM_LIMIT_MAX_NUM,
    ITEM_APPLY_MAX_NUM, ITEM_VALUES_MAX_NUM, ITEM_SOCKET_MAX_NUM
};
use std::ffi::CString;

fn create_mock_item() -> TItemTable {
    let mut item: TItemTable = unsafe { std::mem::zeroed() };
    item.dwVnum = 12345;
    item.bType = 1; // ITEM_TYPE_WEAPON
    item.bSubType = 3; // WEAPON_TWO_HANDED
    item.bSize = 2;
    
    // Set some applies
    item.aApplies[0] = TItemApply { bType: 7, lValue: 15 }; // APPLY_ATT_SPEED
    item.aApplies[1] = TItemApply { bType: 1, lValue: 500 }; // APPLY_MAX_HP
    
    // Set some sockets
    item.alSockets[0] = 100;
    item.alSockets[1] = 200;
    
    // Set some values
    item.alValues[0] = 50;
    item.alValues[1] = 150;
    
    item
}

#[test]
fn test_select_item() {
    let mut mgr = get_item_details_manager().lock().unwrap();
    let item = create_mock_item();
    mgr.insert_item(12345, item.clone());
    drop(mgr);

    assert_eq!(m2_py_item_details_select_item(99999), false);
    assert_eq!(m2_py_item_details_select_item(12345), true);
}

#[test]
fn test_get_affect_normal() {
    let mut mgr = get_item_details_manager().lock().unwrap();
    let mut item = create_mock_item();
    item.bSubType = 0; // WEAPON_SWORD, so no two-handed penalty
    mgr.insert_item(54321, item);
    drop(mgr);

    assert!(m2_py_item_details_select_item(54321));

    let mut out_type: u8 = 0;
    let mut out_value: i32 = 0;

    // Test first apply
    assert!(m2_py_item_get_affect(0, &mut out_type, &mut out_value));
    assert_eq!(out_type, 7);
    assert_eq!(out_value, 15);

    // Test second apply
    assert!(m2_py_item_get_affect(1, &mut out_type, &mut out_value));
    assert_eq!(out_type, 1);
    assert_eq!(out_value, 500);

    // Test out of bounds
    assert!(!m2_py_item_get_affect(ITEM_APPLY_MAX_NUM, &mut out_type, &mut out_value));
}

#[test]
fn test_get_affect_twohanded_penalty() {
    let mut mgr = get_item_details_manager().lock().unwrap();
    let item = create_mock_item(); // Has WEAPON_TWO_HANDED and APPLY_ATT_SPEED
    mgr.insert_item(11111, item);
    drop(mgr);

    m2_py_item_set_twohanded_decrease_value(5);

    assert!(m2_py_item_details_select_item(11111));

    let mut out_type: u8 = 0;
    let mut out_value: i32 = 0;

    // The apply is APPLY_ATT_SPEED = 7, with base value 15.
    // The penalty is 5, so it should return 10.
    assert!(m2_py_item_get_affect(0, &mut out_type, &mut out_value));
    assert_eq!(out_type, 7);
    assert_eq!(out_value, 10);
    
    // Second apply is APPLY_MAX_HP, no penalty should apply.
    assert!(m2_py_item_get_affect(1, &mut out_type, &mut out_value));
    assert_eq!(out_type, 1);
    assert_eq!(out_value, 500);
}

#[test]
fn test_get_socket_valid_and_invalid() {
    let mut mgr = get_item_details_manager().lock().unwrap();
    let item = create_mock_item();
    mgr.insert_item(22222, item);
    drop(mgr);

    assert!(m2_py_item_details_select_item(22222));

    let mut out_value: i32 = 0;

    assert!(m2_py_item_get_socket(0, &mut out_value));
    assert_eq!(out_value, 100);

    assert!(m2_py_item_get_socket(1, &mut out_value));
    assert_eq!(out_value, 200);

    assert!(!m2_py_item_get_socket(ITEM_SOCKET_MAX_NUM, &mut out_value));
}

#[test]
fn test_get_value_valid_and_invalid() {
    let mut mgr = get_item_details_manager().lock().unwrap();
    let item = create_mock_item();
    mgr.insert_item(33333, item);
    drop(mgr);

    assert!(m2_py_item_details_select_item(33333));

    let mut out_value: i32 = 0;

    assert!(m2_py_item_get_value(0, &mut out_value));
    assert_eq!(out_value, 50);

    assert!(m2_py_item_get_value(1, &mut out_value));
    assert_eq!(out_value, 150);

    assert!(!m2_py_item_get_value(ITEM_VALUES_MAX_NUM, &mut out_value));
}

#[test]
fn test_get_item_size() {
    let mut mgr = get_item_details_manager().lock().unwrap();
    let mut item1 = create_mock_item();
    item1.bSize = 2;
    mgr.insert_item(44444, item1);

    let mut item2 = create_mock_item();
    item2.bSize = 0;
    mgr.insert_item(55555, item2);
    drop(mgr);

    let mut out_size: u8 = 0;

    assert!(m2_py_item_details_select_item(44444));
    assert!(m2_py_item_get_item_size(&mut out_size));
    assert_eq!(out_size, 2);

    assert!(m2_py_item_details_select_item(55555));
    assert!(m2_py_item_get_item_size(&mut out_size));
    assert_eq!(out_size, 1); // Fallback to 1
}

#[test]
fn test_get_icon_image_file_name() {
    let mut mgr = get_item_details_manager().lock().unwrap();
    let mut item = create_mock_item();
    item.dwVnum = 11299; // Monks Plate Armour
    mgr.insert_item(11299, item);
    drop(mgr);

    assert!(m2_py_item_details_select_item(11299));

    let mut buf = [0u8; 128];
    assert!(m2_py_item_get_icon_image_file_name(buf.as_mut_ptr() as *mut i8, buf.len()));

    let c_str = unsafe { std::ffi::CStr::from_ptr(buf.as_ptr() as *const i8) };
    assert_eq!(c_str.to_str().unwrap(), "icon/item/11299.tga");
    
    // Test too short buffer
    let mut short_buf = [0u8; 5];
    assert!(!m2_py_item_get_icon_image_file_name(short_buf.as_mut_ptr() as *mut i8, short_buf.len()));
}
