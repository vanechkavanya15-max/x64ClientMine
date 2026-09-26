#[path = "../src/recv_items.rs"]
mod recv_items;

use m2_client_bridge as bridge;

#[allow(unused_imports)]
use bridge::net::gc::item_set::{TPacketGCItemSet, TItemPos as SetItemPos, TPlayerItemAttribute};
#[allow(unused_imports)]
use bridge::net::gc::item_del::{TPacketGCItemDel, TItemPos as DelItemPos};

use bridge::state::game_state::{GLOBAL_GAME_STATE, ItemData};
use std::sync::Mutex;

static TEST_MUTEX: Mutex<()> = Mutex::new(());

#[test]
fn test_handle_item_set_valid() {
    let _guard = TEST_MUTEX.lock().unwrap();
    GLOBAL_GAME_STATE.write().unwrap().reset();

    // Create a mock buffer of expected size 54
    let mut buffer = [0u8; 54];
    
    // Header: 0x0511 (little endian: 0x11, 0x05)
    buffer[0] = 0x11;
    buffer[1] = 0x05;
    // Length: 54
    buffer[2] = 54;
    buffer[3] = 0;
    // Window type: 1 (INVENTORY)
    buffer[4] = 1;
    // Cell: 42
    buffer[5] = 42;
    buffer[6] = 0;
    // Vnum: 12345
    let vnum_bytes = 12345u32.to_le_bytes();
    buffer[7..11].copy_from_slice(&vnum_bytes);
    // Count: 5
    buffer[11] = 5;
    // Flags: 7
    let flags_bytes = 7u32.to_le_bytes();
    buffer[12..16].copy_from_slice(&flags_bytes);
    // Anti Flags: 14
    let anti_flags_bytes = 14u32.to_le_bytes();
    buffer[16..20].copy_from_slice(&anti_flags_bytes);
    // Highlight: 1
    buffer[20] = 1;
    // alSockets: [111, 222, 333]
    let s0 = 111i32.to_le_bytes();
    let s1 = 222i32.to_le_bytes();
    let s2 = 333i32.to_le_bytes();
    buffer[21..25].copy_from_slice(&s0);
    buffer[25..29].copy_from_slice(&s1);
    buffer[29..33].copy_from_slice(&s2);

    // aAttr: [(1, 10), (2, 20), ...]
    for i in 0..7 {
        let offset = 33 + i * 3;
        buffer[offset] = (i + 1) as u8; // bType
        let s_val = ((i + 1) * 10) as i16;
        let s_val_bytes = s_val.to_le_bytes();
        buffer[offset + 1..offset + 3].copy_from_slice(&s_val_bytes);
    }

    assert!(recv_items::handle_item_set(&buffer));

    // Verify state
    let state = GLOBAL_GAME_STATE.read().unwrap();
    let item = state.get_item(1, 42).expect("Item should be in inventory");

    assert_eq!(item.vnum, 12345);
    assert_eq!(item.count, 5);
    assert_eq!(item.flags, 7);
    assert_eq!(item.anti_flags, 14);
    assert_eq!(item.sockets, [111, 222, 333]);
    assert_eq!(item.attrs, [
        (1, 10), (2, 20), (3, 30), (4, 40), (5, 50), (6, 60), (7, 70)
    ]);
}

#[test]
fn test_handle_item_set_invalid_buffer() {
    let _guard = TEST_MUTEX.lock().unwrap();
    GLOBAL_GAME_STATE.write().unwrap().reset();

    let buffer = [0u8; 10]; // Too short
    assert!(!recv_items::handle_item_set(&buffer));

    // Ensure state is unchanged
    let state = GLOBAL_GAME_STATE.read().unwrap();
    assert!(state.get_item(1, 42).is_none());
}

#[test]
fn test_handle_item_del_valid() {
    let _guard = TEST_MUTEX.lock().unwrap();
    GLOBAL_GAME_STATE.write().unwrap().reset();

    // Directly set an item
    {
        let mut state = GLOBAL_GAME_STATE.write().unwrap();
        let item = ItemData {
            vnum: 12345,
            count: 5,
            flags: 0,
            anti_flags: 0,
            sockets: [0; 3],
            attrs: [(0, 0); 7],
        };
        state.set_item(1, 42, item);
    }

    // Ensure item is there
    assert!(GLOBAL_GAME_STATE.read().unwrap().get_item(1, 42).is_some());

    // Create a mock buffer of expected size 7
    let mut buffer = [0u8; 7];
    // Header: 0x0510 (little endian: 0x10, 0x05)
    buffer[0] = 0x10;
    buffer[1] = 0x05;
    // Length: 7
    buffer[2] = 7;
    buffer[3] = 0;
    // Window type: 1 (INVENTORY)
    buffer[4] = 1;
    // Cell: 42
    buffer[5] = 42;
    buffer[6] = 0;

    assert!(recv_items::handle_item_del(&buffer));

    // Verify item is removed
    let state = GLOBAL_GAME_STATE.read().unwrap();
    assert!(state.get_item(1, 42).is_none());
}

#[test]
fn test_handle_item_del_invalid_buffer() {
    let _guard = TEST_MUTEX.lock().unwrap();
    GLOBAL_GAME_STATE.write().unwrap().reset();

    let buffer = [0u8; 4]; // Too short
    assert!(!recv_items::handle_item_del(&buffer));
}
