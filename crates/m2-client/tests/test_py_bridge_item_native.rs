use m2_client::item_proto_binary_loader::{
    TItemApply, TItemLimit, TItemTable, ITEM_APPLY_MAX_NUM, ITEM_LIMIT_MAX_NUM, ITEM_NAME_MAX_LEN,
    ITEM_SOCKET_MAX_NUM, ITEM_VALUES_MAX_NUM,
};
use m2_client::py_bridge_item_native::{
    get_item_native_manager, m2_py_item_native_get_apply,
    m2_py_item_native_get_icon_image_file_name, m2_py_item_native_get_ibuy_price,
    m2_py_item_native_get_isell_price, m2_py_item_native_get_item_name,
    m2_py_item_native_get_item_size, m2_py_item_native_get_item_sub_type,
    m2_py_item_native_get_item_type, m2_py_item_native_get_limit, m2_py_item_native_select_item,
};
use std::ffi::CStr;
use std::os::raw::c_char;

fn create_dummy_item(
    vnum: u32,
    name: &str,
    b_type: u8,
    b_sub_type: u8,
    b_size: u8,
    buy_price: u32,
    sell_price: u32,
    limits: [(u8, i32); ITEM_LIMIT_MAX_NUM],
    applies: [(u8, i32); ITEM_APPLY_MAX_NUM],
) -> TItemTable {
    let mut sz_name = [0u8; ITEM_NAME_MAX_LEN + 1];
    let name_bytes = name.as_bytes();
    let len = std::cmp::min(name_bytes.len(), ITEM_NAME_MAX_LEN);
    sz_name[..len].copy_from_slice(&name_bytes[..len]);

    let mut a_limits = [TItemLimit {
        bType: 0,
        lValue: 0,
    }; ITEM_LIMIT_MAX_NUM];
    for (i, &(limit_type, limit_value)) in limits.iter().enumerate() {
        a_limits[i] = TItemLimit {
            bType: limit_type,
            lValue: limit_value,
        };
    }

    let mut a_applies = [TItemApply {
        bType: 0,
        lValue: 0,
    }; ITEM_APPLY_MAX_NUM];
    for (i, &(apply_type, apply_value)) in applies.iter().enumerate() {
        a_applies[i] = TItemApply {
            bType: apply_type,
            lValue: apply_value,
        };
    }

    TItemTable {
        dwVnum: vnum,
        dwVnumRange: 0,
        szName: sz_name,
        szLocaleName: sz_name,
        bType: b_type,
        bSubType: b_sub_type,
        bWeight: 0,
        bSize: b_size,
        dwAntiFlags: 0,
        dwFlags: 0,
        dwWearFlags: 0,
        dwImmuneFlag: 0,
        dwIBuyItemPrice: buy_price,
        dwISellItemPrice: sell_price,
        aLimits: a_limits,
        aApplies: a_applies,
        alValues: [0; ITEM_VALUES_MAX_NUM],
        alSockets: [0; ITEM_SOCKET_MAX_NUM],
        dwRefinedVnum: 0,
        wRefineSet: 0,
        bAlterToMagicItemPct: 0,
        bSpecular: 0,
        bGainSocketPct: 0,
    }
}

fn setup_test_items() {
    let mut mgr = get_item_native_manager().lock().unwrap();

    // Sword (Weapon, Sword type)
    mgr.insert_item(
        10,
        create_dummy_item(
            10,
            "Sword",
            1, // Weapon
            0, // Sword
            2,
            100,
            50,
            [(1, 10), (0, 0)],
            [(1, 5), (0, 0), (0, 0)],
        ),
    );

    // Armor
    mgr.insert_item(
        11299,
        create_dummy_item(
            11299,
            "Monk Plate Armor",
            2, // Armor
            0, // Body
            2,
            5000,
            2500,
            [(1, 1), (0, 0)],
            [(1, 1000), (4, 10), (0, 0)],
        ),
    );

    // Potion
    mgr.insert_item(
        27001,
        create_dummy_item(
            27001,
            "Red Potion(S)",
            3, // Use
            0,
            1,
            32,
            16,
            [(0, 0), (0, 0)],
            [(0, 0), (0, 0), (0, 0)],
        ),
    );
}

#[test]
fn test_select_item() {
    setup_test_items();
    assert!(m2_py_item_native_select_item(10));
    assert!(m2_py_item_native_select_item(11299));
    assert!(m2_py_item_native_select_item(27001));
    assert!(!m2_py_item_native_select_item(999999)); // Invalid item
}

#[test]
fn test_get_item_name() {
    setup_test_items();
    assert!(m2_py_item_native_select_item(10));

    let mut buf = vec![0i8; 64];
    assert!(m2_py_item_native_get_item_name(
        buf.as_mut_ptr() as *mut c_char,
        buf.len()
    ));

    let name = unsafe { CStr::from_ptr(buf.as_ptr() as *const c_char) }
        .to_str()
        .unwrap();
    assert_eq!(name, "Sword");
}

#[test]
fn test_get_item_name_buffer_too_small() {
    setup_test_items();
    assert!(m2_py_item_native_select_item(11299));

    let mut buf = vec![0i8; 5]; // "Monk Plate Armor" is 16 chars
    assert!(!m2_py_item_native_get_item_name(
        buf.as_mut_ptr() as *mut c_char,
        buf.len()
    ));
}

#[test]
fn test_get_item_properties() {
    setup_test_items();
    assert!(m2_py_item_native_select_item(11299)); // Armor

    assert_eq!(m2_py_item_native_get_item_type(), 2);
    assert_eq!(m2_py_item_native_get_item_sub_type(), 0);
    assert_eq!(m2_py_item_native_get_item_size(), 2);
    assert_eq!(m2_py_item_native_get_ibuy_price(), 5000);
    assert_eq!(m2_py_item_native_get_isell_price(), 2500);
}

#[test]
fn test_get_item_size_fallback() {
    let mut mgr = get_item_native_manager().lock().unwrap();
    mgr.insert_item(
        999,
        create_dummy_item(
            999,
            "Zero Size Item",
            0,
            0,
            0, // Size 0 should fallback to 1
            0,
            0,
            [(0, 0), (0, 0)],
            [(0, 0), (0, 0), (0, 0)],
        ),
    );
    drop(mgr); // Release lock before calling FFI

    assert!(m2_py_item_native_select_item(999));
    assert_eq!(m2_py_item_native_get_item_size(), 1);
}

#[test]
fn test_get_icon_image_file_name() {
    setup_test_items();
    assert!(m2_py_item_native_select_item(10));

    let mut buf = vec![0i8; 64];
    assert!(m2_py_item_native_get_icon_image_file_name(
        buf.as_mut_ptr() as *mut c_char,
        buf.len()
    ));

    let path = unsafe { CStr::from_ptr(buf.as_ptr() as *const c_char) }
        .to_str()
        .unwrap();
    assert_eq!(path, "icon/item/00010.tga");
}

#[test]
fn test_get_limit() {
    setup_test_items();
    assert!(m2_py_item_native_select_item(10)); // Sword has limits: [(1, 10), (0, 0)]

    let mut limit_type: u8 = 0;
    let mut limit_value: i32 = 0;

    assert!(m2_py_item_native_get_limit(
        0,
        &mut limit_type,
        &mut limit_value
    ));
    assert_eq!(limit_type, 1);
    assert_eq!(limit_value, 10);

    assert!(m2_py_item_native_get_limit(
        1,
        &mut limit_type,
        &mut limit_value
    ));
    assert_eq!(limit_type, 0);
    assert_eq!(limit_value, 0);

    // Out of bounds
    assert!(!m2_py_item_native_get_limit(
        2,
        &mut limit_type,
        &mut limit_value
    ));
}

#[test]
fn test_get_apply() {
    setup_test_items();
    assert!(m2_py_item_native_select_item(11299)); // Armor applies: [(1, 1000), (4, 10), (0, 0)]

    let mut apply_type: u8 = 0;
    let mut apply_value: i32 = 0;

    assert!(m2_py_item_native_get_apply(
        0,
        &mut apply_type,
        &mut apply_value
    ));
    assert_eq!(apply_type, 1);
    assert_eq!(apply_value, 1000);

    assert!(m2_py_item_native_get_apply(
        1,
        &mut apply_type,
        &mut apply_value
    ));
    assert_eq!(apply_type, 4);
    assert_eq!(apply_value, 10);

    assert!(m2_py_item_native_get_apply(
        2,
        &mut apply_type,
        &mut apply_value
    ));
    assert_eq!(apply_type, 0);
    assert_eq!(apply_value, 0);

    // Out of bounds
    assert!(!m2_py_item_native_get_apply(
        3,
        &mut apply_type,
        &mut apply_value
    ));
}