use m2_client::py_bridge_item_limits::{m2_py_item_get_limit_type, m2_py_item_get_limit_value, m2_py_item_is_wearable_flag, m2_py_item_set_use_sound_file_name, m2_py_item_set_drop_sound_file_name};
use m2_client::py_bridge_item::{m2_py_item_select_item, ItemLimit, ItemApply, register_item_proto};
use m2_client::item_proto_loader::ItemProto;
use std::ffi::CString;

#[test]
fn test_item_get_limit_type_and_value() {
    let mut limits = [ItemLimit::default(); 2];
    limits[0] = ItemLimit { limit_type: 1, limit_value: 30 };
    limits[1] = ItemLimit { limit_type: 2, limit_value: 15 };
    let item = ItemProto {
        vnum: 100,
        name: "Test Limit Item".to_string(),
        item_type: 1,
        item_sub_type: 0,
        buy_price: 10,
        sell_price: 5,
        icon_image: "test.tga".to_string(),
        limits,
        applies: [ItemApply::default(); 3],
        wear_flags: 0,
    };
    register_item_proto(item);
    m2_py_item_select_item(100);

    assert_eq!(m2_py_item_get_limit_type(0), 1);
    assert_eq!(m2_py_item_get_limit_value(0), 30);
    assert_eq!(m2_py_item_get_limit_type(1), 2);
    assert_eq!(m2_py_item_get_limit_value(1), 15);
    // Out of bounds
    assert_eq!(m2_py_item_get_limit_type(2), 0);
    assert_eq!(m2_py_item_get_limit_value(2), 0);
}

#[test]
fn test_item_is_wearable_flag() {
    let item = ItemProto {
        vnum: 101,
        name: "Test Wearable Item".to_string(),
        item_type: 1,
        item_sub_type: 0,
        buy_price: 10,
        sell_price: 5,
        icon_image: "test.tga".to_string(),
        limits: [ItemLimit::default(); 2],
        applies: [ItemApply::default(); 3],
        wear_flags: 5, // 1 | 4
    };
    register_item_proto(item);
    m2_py_item_select_item(101);

    assert!(m2_py_item_is_wearable_flag(1));
    assert!(!m2_py_item_is_wearable_flag(2));
    assert!(m2_py_item_is_wearable_flag(4));
    assert!(!m2_py_item_is_wearable_flag(8));
}

#[test]
fn test_item_set_use_sound_file_name() {
    let file_name = CString::new("sound/use_potion.wav").unwrap();
    m2_py_item_set_use_sound_file_name(6, file_name.as_ptr());
    let mgr = m2_client::item_manager::get_item_manager().lock().unwrap();
    assert_eq!(mgr.use_sound_filenames.get(&6).unwrap(), "sound/use_potion.wav");
    
    // Test invalid limit (> 7)
    let invalid_file_name = CString::new("invalid.wav").unwrap();
    m2_py_item_set_use_sound_file_name(8, invalid_file_name.as_ptr());
    assert!(mgr.use_sound_filenames.get(&8).is_none());
}

#[test]
fn test_item_set_drop_sound_file_name() {
    let file_name = CString::new("sound/drop_sword.wav").unwrap();
    m2_py_item_set_drop_sound_file_name(2, file_name.as_ptr());
    let mgr = m2_client::item_manager::get_item_manager().lock().unwrap();
    assert_eq!(mgr.drop_sound_filenames.get(&2).unwrap(), "sound/drop_sword.wav");

    // Test invalid limit (> 4)
    let invalid_file_name = CString::new("invalid_drop.wav").unwrap();
    m2_py_item_set_drop_sound_file_name(5, invalid_file_name.as_ptr());
    assert!(mgr.drop_sound_filenames.get(&5).is_none());
}

