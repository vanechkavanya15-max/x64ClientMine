use std::sync::{Mutex, OnceLock};

use m2_client::py_bridge_item_native::get_item_native_manager;
use m2_client::inventory_grid_v2::{get_inventory_sys, WindowType, ItemData};
use m2_client::item_proto_binary_loader::{TItemTable, TItemLimit, TItemApply};
use m2_client::py_bridge_player_items::{
    ATTACH_METIN_CANT, ATTACH_METIN_OK, ATTACH_METIN_NOT_MATCHABLE_ITEM,
    ATTACH_METIN_NO_MATCHABLE_SOCKET, ATTACH_METIN_NOT_EXIST_GOLD_SOCKET,
    ATTACH_METIN_CANT_ATTACH_TO_EQUIPMENT, ITEM_TYPE_ROD
};
use m2_client::game_item_refine_socket_kd_manager::{
    m2_py_player_can_attach_metin,
    METIN_SOCKET_TYPE_NONE, METIN_SOCKET_TYPE_SILVER, METIN_SOCKET_TYPE_GOLD,
    METIN_NORMAL, METIN_GOLD
};
use m2_client::item_sockets::{ItemSockets, SOCKET_BROKEN, SOCKET_EMPTY};

static TEST_MUTEX: OnceLock<Mutex<()>> = OnceLock::new();

fn lock_test() -> std::sync::MutexGuard<'static, ()> {
    TEST_MUTEX.get_or_init(|| Mutex::new(())).lock().unwrap()
}

fn clear_state() {
    let mut inv = get_inventory_sys().lock().unwrap();
    for cell in 0..200 {
        inv.del_item(WindowType::INVENTORY, cell);
    }
    
    let mut mgr = get_item_native_manager().lock().unwrap();
    mgr.items.clear();
}

fn create_mock_item_table(vnum: u32, b_type: u8, sub_type: u8, wear_flags: u32) -> TItemTable {
    TItemTable {
        dwVnum: vnum,
        dwVnumRange: 0,
        szName: [0; 65],
        szLocaleName: [0; 65],
        bType: b_type,
        bSubType: sub_type,
        bWeight: 0,
        bSize: 1,
        dwAntiFlags: 0,
        dwFlags: 0,
        dwWearFlags: wear_flags,
        dwImmuneFlag: 0,
        dwIBuyItemPrice: 0,
        dwISellItemPrice: 0,
        aLimits: [TItemLimit { bType: 0, lValue: 0 }; 2],
        aApplies: [TItemApply { bType: 0, lValue: 0 }; 3],
        alValues: [0; 6],
        alSockets: [0; 3],
        dwRefinedVnum: 0,
        wRefineSet: 0,
        bAlterToMagicItemPct: 0,
        bSpecular: 0,
        bGainSocketPct: 0,
    }
}

#[test]
fn test_attach_metin_to_equipment_fails() {
    let _guard = lock_test();
    clear_state();

    let result = m2_py_player_can_attach_metin(1000, WindowType::EQUIPMENT as u8, 0);
    assert_eq!(result, ATTACH_METIN_CANT_ATTACH_TO_EQUIPMENT);
}

#[test]
fn test_attach_metin_not_matchable_item() {
    let _guard = lock_test();
    clear_state();

    let mut mgr = get_item_native_manager().lock().unwrap();
    mgr.insert_item(5000, create_mock_item_table(5000, 10, METIN_NORMAL, 1)); 
    mgr.insert_item(100, create_mock_item_table(100, 1, 0, 2)); 
    drop(mgr);
    
    let mut inv = get_inventory_sys().lock().unwrap();
    inv.set_item(WindowType::INVENTORY, 0, ItemData {
        vnum: 100,
        count: 1,
        sockets: [METIN_SOCKET_TYPE_SILVER, METIN_SOCKET_TYPE_NONE, METIN_SOCKET_TYPE_NONE],
        ..Default::default()
    });
    drop(inv);

    let result = m2_py_player_can_attach_metin(5000, WindowType::INVENTORY as u8, 0);
    assert_eq!(result, ATTACH_METIN_NOT_MATCHABLE_ITEM);
}

#[test]
fn test_attach_metin_cant_for_rod() {
    let _guard = lock_test();
    clear_state();

    let mut mgr = get_item_native_manager().lock().unwrap();
    mgr.insert_item(5000, create_mock_item_table(5000, 10, METIN_NORMAL, 3)); 
    mgr.insert_item(100, create_mock_item_table(100, ITEM_TYPE_ROD, 0, 3)); 
    drop(mgr);
    
    let mut inv = get_inventory_sys().lock().unwrap();
    inv.set_item(WindowType::INVENTORY, 0, ItemData {
        vnum: 100,
        count: 1,
        sockets: [METIN_SOCKET_TYPE_SILVER, METIN_SOCKET_TYPE_NONE, METIN_SOCKET_TYPE_NONE],
        ..Default::default()
    });
    drop(inv);

    let result = m2_py_player_can_attach_metin(5000, WindowType::INVENTORY as u8, 0);
    assert_eq!(result, ATTACH_METIN_CANT);
}

#[test]
fn test_attach_metin_ok_normal() {
    let _guard = lock_test();
    clear_state();

    let mut mgr = get_item_native_manager().lock().unwrap();
    mgr.insert_item(5000, create_mock_item_table(5000, 10, METIN_NORMAL, 1)); 
    mgr.insert_item(100, create_mock_item_table(100, 1, 0, 1)); 
    drop(mgr);
    
    let mut inv = get_inventory_sys().lock().unwrap();
    inv.set_item(WindowType::INVENTORY, 0, ItemData {
        vnum: 100,
        count: 1,
        sockets: [METIN_SOCKET_TYPE_SILVER, METIN_SOCKET_TYPE_NONE, METIN_SOCKET_TYPE_NONE],
        ..Default::default()
    });
    drop(inv);

    let result = m2_py_player_can_attach_metin(5000, WindowType::INVENTORY as u8, 0);
    assert_eq!(result, ATTACH_METIN_OK);
}

#[test]
fn test_attach_metin_ok_gold() {
    let _guard = lock_test();
    clear_state();

    let mut mgr = get_item_native_manager().lock().unwrap();
    mgr.insert_item(5000, create_mock_item_table(5000, 10, METIN_GOLD, 1)); 
    mgr.insert_item(100, create_mock_item_table(100, 1, 0, 1)); 
    drop(mgr);
    
    let mut inv = get_inventory_sys().lock().unwrap();
    inv.set_item(WindowType::INVENTORY, 0, ItemData {
        vnum: 100,
        count: 1,
        sockets: [METIN_SOCKET_TYPE_NONE, METIN_SOCKET_TYPE_GOLD, METIN_SOCKET_TYPE_NONE],
        ..Default::default()
    });
    drop(inv);

    let result = m2_py_player_can_attach_metin(5000, WindowType::INVENTORY as u8, 0);
    assert_eq!(result, ATTACH_METIN_OK);
}

#[test]
fn test_attach_metin_not_exist_gold_socket() {
    let _guard = lock_test();
    clear_state();

    let mut mgr = get_item_native_manager().lock().unwrap();
    mgr.insert_item(5000, create_mock_item_table(5000, 10, METIN_GOLD, 1)); 
    mgr.insert_item(100, create_mock_item_table(100, 1, 0, 1)); 
    drop(mgr);
    
    let mut inv = get_inventory_sys().lock().unwrap();
    inv.set_item(WindowType::INVENTORY, 0, ItemData {
        vnum: 100,
        count: 1,
        sockets: [METIN_SOCKET_TYPE_SILVER, METIN_SOCKET_TYPE_NONE, METIN_SOCKET_TYPE_NONE],
        ..Default::default()
    });
    drop(inv);

    let result = m2_py_player_can_attach_metin(5000, WindowType::INVENTORY as u8, 0);
    assert_eq!(result, ATTACH_METIN_NOT_EXIST_GOLD_SOCKET);
}

#[test]
fn test_attach_metin_no_matchable_socket() {
    let _guard = lock_test();
    clear_state();

    let mut mgr = get_item_native_manager().lock().unwrap();
    mgr.insert_item(5000, create_mock_item_table(5000, 10, METIN_NORMAL, 1)); 
    mgr.insert_item(100, create_mock_item_table(100, 1, 0, 1)); 
    drop(mgr);
    
    let mut inv = get_inventory_sys().lock().unwrap();
    inv.set_item(WindowType::INVENTORY, 0, ItemData {
        vnum: 100,
        count: 1,
        sockets: [METIN_SOCKET_TYPE_NONE, METIN_SOCKET_TYPE_NONE, METIN_SOCKET_TYPE_NONE],
        ..Default::default()
    });
    drop(inv);

    let result = m2_py_player_can_attach_metin(5000, WindowType::INVENTORY as u8, 0);
    assert_eq!(result, ATTACH_METIN_NO_MATCHABLE_SOCKET);
}

#[test]
fn test_clean_broken_socket_success() {
    let _guard = lock_test();

    let mut sockets = ItemSockets::new();
    
    sockets.sockets[0] = 5000;
    sockets.sockets[1] = SOCKET_BROKEN;
    sockets.sockets[2] = SOCKET_BROKEN;

    let result = sockets.clean_broken_sockets();
    
    assert!(result);
    assert_eq!(sockets.sockets[0], 5000);
    assert_eq!(sockets.sockets[1], SOCKET_EMPTY);
    assert_eq!(sockets.sockets[2], SOCKET_EMPTY);
}
