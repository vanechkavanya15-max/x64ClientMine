use std::mem::size_of;
use std::ptr;
use std::sync::Mutex;

use m2_client::recv_items_payload::*;
use m2_client_bridge::state::game_state::{GLOBAL_GAME_STATE, ItemData};

static TEST_MUTEX: Mutex<()> = Mutex::new(());

#[test]
fn test_struct_sizes() {
    assert_eq!(size_of::<TPacketGCItemSet>(), 54);
    assert_eq!(size_of::<TPacketGCItemDel>(), 7);
    assert_eq!(size_of::<TPacketGCItemUpdate>(), 39);
}

fn reset_game_state() {
    if let Ok(mut state) = GLOBAL_GAME_STATE.write() {
        state.reset();
    }
}

#[test]
fn test_handle_item_set_success() {
    let _guard = TEST_MUTEX.lock().unwrap();
    reset_game_state();

    let mut packet = [0u8; 54];
    
    // header
    packet[0..2].copy_from_slice(&HEADER_GC_ITEM_SET.to_le_bytes());
    // length
    packet[2..4].copy_from_slice(&54u16.to_le_bytes());
    // length
    // pos.window_type
    packet[4] = 1; // INVENTORY
    // pos.cell
    packet[5..7].copy_from_slice(&10u16.to_le_bytes());
    // vnum
    packet[7..11].copy_from_slice(&11299u32.to_le_bytes()); // Zbroja
    // count
    packet[11] = 1;
    // flags
    packet[12..16].copy_from_slice(&0u32.to_le_bytes());
    // anti_flags
    packet[16..20].copy_from_slice(&0u32.to_le_bytes());
    // highlight
    packet[20] = 0;
    
    // sockets[3]
    for i in 0..3 {
        let offset = 21 + i * 4;
        packet[offset..offset+4].copy_from_slice(&(i as i32 + 1).to_le_bytes());
    }
    
    // attrs[7]
    for i in 0..7 {
        let offset = 33 + i * 3;
        packet[offset] = (i as u8) + 1; // bType
        packet[offset+1..offset+3].copy_from_slice(&((i as i16) * 10).to_le_bytes()); // sValue
    }

    let result = handle_item_set(&packet);
    assert!(result.is_ok());

    if let Ok(state) = GLOBAL_GAME_STATE.read() {
        let item = state.get_item(1, 10).expect("Item should be in inventory");
        assert_eq!(item.vnum, 11299);
        assert_eq!(item.count, 1);
        assert_eq!(item.sockets, [1, 2, 3]);
        
        for i in 0..7 {
            assert_eq!(item.attrs[i], (i as u8 + 1, (i as i16) * 10));
        }
    } else {
        panic!("Failed to lock GLOBAL_GAME_STATE");
    }
}

#[test]
fn test_handle_item_del_success() {
    let _guard = TEST_MUTEX.lock().unwrap();
    reset_game_state();

    if let Ok(mut state) = GLOBAL_GAME_STATE.write() {
        state.set_item(1, 5, ItemData::new(50200, 200));
    }

    let mut packet = [0u8; 7];
    packet[0..2].copy_from_slice(&HEADER_GC_ITEM_DEL.to_le_bytes());
    packet[2..4].copy_from_slice(&7u16.to_le_bytes());
    packet[4] = 1; // INVENTORY
    packet[5..7].copy_from_slice(&5u16.to_le_bytes());

    let result = handle_item_del(&packet);
    assert!(result.is_ok());

    if let Ok(state) = GLOBAL_GAME_STATE.read() {
        assert!(state.get_item(1, 5).is_none());
    } else {
        panic!("Failed to lock GLOBAL_GAME_STATE");
    }
}

#[test]
fn test_handle_item_update_success() {
    let _guard = TEST_MUTEX.lock().unwrap();
    reset_game_state();

    if let Ok(mut state) = GLOBAL_GAME_STATE.write() {
        let mut initial_item = ItemData::new(27003, 200); // Red Potion(L)
        initial_item.sockets = [0, 0, 0];
        state.set_item(1, 20, initial_item);
    }

    let mut packet = [0u8; 39];
    packet[0..2].copy_from_slice(&HEADER_GC_ITEM_UPDATE.to_le_bytes());
    packet[2] = 1; // window
    packet[3..5].copy_from_slice(&20u16.to_le_bytes()); // cell
    packet[5] = 199; // new count
    
    // update socket 0
    packet[6..10].copy_from_slice(&999i32.to_le_bytes());
    // socket 1
    packet[10..14].copy_from_slice(&0i32.to_le_bytes());
    // socket 2
    packet[14..18].copy_from_slice(&0i32.to_le_bytes());
    
    // clear attrs
    for i in 0..7 {
        let offset = 18 + i * 3;
        packet[offset] = 0;
        packet[offset+1..offset+3].copy_from_slice(&0i16.to_le_bytes());
    }
    
    // Set first attr
    packet[18] = 5; // bType
    packet[19..21].copy_from_slice(&10i16.to_le_bytes());

    let result = handle_item_update(&packet);
    assert!(result.is_ok());

    if let Ok(state) = GLOBAL_GAME_STATE.read() {
        let item = state.get_item(1, 20).expect("Item should be in inventory");
        assert_eq!(item.vnum, 27003); // vnum unchanged
        assert_eq!(item.count, 199);
        assert_eq!(item.sockets[0], 999);
        assert_eq!(item.attrs[0], (5, 10));
    } else {
        panic!("Failed to lock GLOBAL_GAME_STATE");
    }
}

#[test]
fn test_invalid_header_fails() {
    let mut packet = [0u8; 54];
    packet[0..2].copy_from_slice(&0xFFFFu16.to_le_bytes()); // Invalid header
    
    let result = handle_item_set(&packet);
    assert_eq!(result, Err(PacketError::InvalidHeader));
}

#[test]
fn test_buffer_too_small_fails() {
    let packet = [0u8; 10]; // Smaller than 54
    
    let result = handle_item_set(&packet);
    assert_eq!(result, Err(PacketError::BufferTooSmall));
}
