use m2_client::py_bridge_item::*;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::sync::Once;
use serial_test::serial;

static INIT: Once = Once::new();

fn setup() {
    INIT.call_once(|| {
        let mut limits = [ItemLimit::default(); ITEM_LIMIT_MAX_NUM];
        limits[0] = ItemLimit { limit_type: 1, limit_value: 30 };
        
        let mut applies = [ItemApply::default(); ITEM_APPLY_MAX_NUM];
        applies[0] = ItemApply { apply_type: 1, apply_value: 1000 };
        applies[1] = ItemApply { apply_type: 5, apply_value: 15 };

        let item = ItemProto {
            vnum: 11299,
            name: "Dragon God Armor+9".to_string(),
            item_type: 2, // ITEM_TYPE_ARMOR
            item_sub_type: 0, // ARMOR_BODY
            buy_price: 1000000,
            sell_price: 250000,
            icon_image: "icon/item/11299.tga".to_string(),
            limits,
            applies,
        };

        register_item_proto(item);
    });
}

#[test]
#[serial]
fn test_select_item() {
    setup();
    m2_py_item_select_item(11299);
    
    let mut buf = vec![0u8; 100];
    let ok = m2_py_item_get_item_name(buf.as_mut_ptr() as *mut c_char, buf.len());
    assert!(ok);
    
    let name = unsafe { CStr::from_ptr(buf.as_ptr() as *const c_char).to_str().unwrap() };
    assert_eq!(name, "Dragon God Armor+9");
}

#[test]
#[serial]
fn test_get_item_types() {
    setup();
    m2_py_item_select_item(11299);
    
    let item_type = m2_py_item_get_item_type();
    assert_eq!(item_type, 2);
    
    let item_sub_type = m2_py_item_get_item_sub_type();
    assert_eq!(item_sub_type, 0);
}

#[test]
#[serial]
fn test_get_item_prices() {
    setup();
    m2_py_item_select_item(11299);
    
    let buy_price = m2_py_item_get_ibuy_item_price();
    assert_eq!(buy_price, 1000000);
    
    let sell_price = m2_py_item_get_isell_item_price();
    assert_eq!(sell_price, 250000);
}

#[test]
#[serial]
fn test_get_icon_image() {
    setup();
    m2_py_item_select_item(11299);
    
    let mut buf = vec![0u8; 100];
    let ok = m2_py_item_get_icon_image(buf.as_mut_ptr() as *mut c_char, buf.len());
    assert!(ok);
    
    let icon = unsafe { CStr::from_ptr(buf.as_ptr() as *const c_char).to_str().unwrap() };
    assert_eq!(icon, "icon/item/11299.tga");
}

#[test]
#[serial]
fn test_get_limit() {
    setup();
    m2_py_item_select_item(11299);
    
    let mut limit_type: u8 = 0;
    let mut limit_value: i32 = 0;
    
    let ok = m2_py_item_get_limit(0, &mut limit_type, &mut limit_value);
    assert!(ok);
    assert_eq!(limit_type, 1);
    assert_eq!(limit_value, 30);
    
    // Out of bounds
    let ok = m2_py_item_get_limit(2, &mut limit_type, &mut limit_value);
    assert!(!ok);
}

#[test]
#[serial]
fn test_get_apply() {
    setup();
    m2_py_item_select_item(11299);
    
    let mut apply_type: u8 = 0;
    let mut apply_value: i16 = 0;
    
    let ok = m2_py_item_get_apply(0, &mut apply_type, &mut apply_value);
    assert!(ok);
    assert_eq!(apply_type, 1);
    assert_eq!(apply_value, 1000);
    
    let ok = m2_py_item_get_apply(1, &mut apply_type, &mut apply_value);
    assert!(ok);
    assert_eq!(apply_type, 5);
    assert_eq!(apply_value, 15);
    
    // Out of bounds
    let ok = m2_py_item_get_apply(3, &mut apply_type, &mut apply_value);
    assert!(!ok);
}

#[test]
#[serial]
fn test_select_non_existent_item() {
    setup();
    m2_py_item_select_item(99999);
    
    let item_type = m2_py_item_get_item_type();
    assert_eq!(item_type, 0);
    
    let mut buf = vec![0u8; 100];
    let ok = m2_py_item_get_item_name(buf.as_mut_ptr() as *mut c_char, buf.len());
    assert!(!ok);
}

#[test]
#[serial]
fn test_get_item_name_buffer_too_small() {
    setup();
    m2_py_item_select_item(11299);
    
    let mut buf = vec![0u8; 5]; // "Dragon God Armor+9" is longer than 5
    let ok = m2_py_item_get_item_name(buf.as_mut_ptr() as *mut c_char, buf.len());
    assert!(!ok);
}
