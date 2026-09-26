use m2_client::recv_quickslot_sync_v2::*;
use std::sync::Mutex;

static TEST_LOCK: Mutex<()> = Mutex::new(());

#[test]
fn test_process_quickslot_add() {
    let _guard = TEST_LOCK.lock().unwrap();
    clear_quickslot_registry();

    let packet = TPacketGCQuickSlotAdd {
        header: 100,
        length: 6,
        pos: 5,
        slot: TQuickSlot {
            slot_type: SLOT_TYPE_SKILL,
            slot_position: 12,
        },
    };

    let ptr = &packet as *const _ as *const u8;
    let result = process_quickslot_add(ptr);

    assert!(result);

    let res = m2_quickslot_get_v2(5);
    assert_eq!(res.slot_type, SLOT_TYPE_SKILL);
    assert_eq!(res.slot_position, 12);
}

#[test]
fn test_process_quickslot_add_out_of_bounds() {
    let _guard = TEST_LOCK.lock().unwrap();
    clear_quickslot_registry();

    let packet = TPacketGCQuickSlotAdd {
        header: 100,
        length: 6,
        pos: QUICKSLOT_MAX_NUM as u8, // Out of bounds
        slot: TQuickSlot {
            slot_type: SLOT_TYPE_SKILL,
            slot_position: 12,
        },
    };

    let ptr = &packet as *const _ as *const u8;
    let result = process_quickslot_add(ptr);

    assert!(!result);
}

#[test]
fn test_process_quickslot_del() {
    let _guard = TEST_LOCK.lock().unwrap();
    clear_quickslot_registry();

    // First add
    let add_packet = TPacketGCQuickSlotAdd {
        header: 100,
        length: 6,
        pos: 7,
        slot: TQuickSlot {
            slot_type: SLOT_TYPE_INVENTORY,
            slot_position: 99,
        },
    };
    let ptr = &add_packet as *const _ as *const u8;
    process_quickslot_add(ptr);

    // Verify addition
    let res = m2_quickslot_get_v2(7);
    assert_eq!(res.slot_type, SLOT_TYPE_INVENTORY);
    
    // Now delete
    let del_packet = TPacketGCQuickSlotDel {
        header: 101,
        length: 5,
        pos: 7,
    };
    let del_ptr = &del_packet as *const _ as *const u8;
    let result = process_quickslot_del(del_ptr);

    assert!(result);

    let res = m2_quickslot_get_v2(7);
    assert_eq!(res.slot_type, SLOT_TYPE_NONE);
    assert_eq!(res.slot_position, 0);
}

#[test]
fn test_process_quickslot_swap() {
    let _guard = TEST_LOCK.lock().unwrap();
    clear_quickslot_registry();

    // Add to pos 2
    let add_packet1 = TPacketGCQuickSlotAdd {
        header: 100,
        length: 6,
        pos: 2,
        slot: TQuickSlot {
            slot_type: SLOT_TYPE_SKILL,
            slot_position: 1,
        },
    };
    process_quickslot_add(&add_packet1 as *const _ as *const u8);

    // Add to pos 3
    let add_packet2 = TPacketGCQuickSlotAdd {
        header: 100,
        length: 6,
        pos: 3,
        slot: TQuickSlot {
            slot_type: SLOT_TYPE_EMOTION,
            slot_position: 5,
        },
    };
    process_quickslot_add(&add_packet2 as *const _ as *const u8);

    // Swap 2 and 3
    let swap_packet = TPacketGCQuickSlotSwap {
        header: 102,
        length: 6,
        pos: 2,
        change_pos: 3,
    };
    let result = process_quickslot_swap(&swap_packet as *const _ as *const u8);

    assert!(result);

    let res2 = m2_quickslot_get_v2(2);
    assert_eq!(res2.slot_type, SLOT_TYPE_EMOTION);
    assert_eq!(res2.slot_position, 5);

    let res3 = m2_quickslot_get_v2(3);
    assert_eq!(res3.slot_type, SLOT_TYPE_SKILL);
    assert_eq!(res3.slot_position, 1);
}

#[test]
fn test_process_quickslot_swap_out_of_bounds() {
    let _guard = TEST_LOCK.lock().unwrap();
    clear_quickslot_registry();

    let swap_packet = TPacketGCQuickSlotSwap {
        header: 102,
        length: 6,
        pos: 2,
        change_pos: QUICKSLOT_MAX_NUM as u8,
    };
    let result = process_quickslot_swap(&swap_packet as *const _ as *const u8);

    assert!(!result);
}

#[test]
fn test_m2_quickslot_get_v2_out_of_bounds() {
    let _guard = TEST_LOCK.lock().unwrap();
    clear_quickslot_registry();

    let res = m2_quickslot_get_v2(QUICKSLOT_MAX_NUM as u8 + 10);
    assert_eq!(res.slot_type, SLOT_TYPE_NONE);
    assert_eq!(res.slot_position, 0);
}

#[test]
fn test_unaligned_read_safety() {
    let _guard = TEST_LOCK.lock().unwrap();
    clear_quickslot_registry();

    // Create a larger byte array and write unaligned
    let mut buffer = [0u8; 32];
    
    let packet = TPacketGCQuickSlotAdd {
        header: 100,
        length: 6,
        pos: 5,
        slot: TQuickSlot {
            slot_type: SLOT_TYPE_SKILL,
            slot_position: 12,
        },
    };

    unsafe {
        // write to buffer at offset 1 to ensure unaligned pointer
        std::ptr::write_unaligned(buffer.as_mut_ptr().add(1) as *mut TPacketGCQuickSlotAdd, packet);
    }

    // Process from offset 1
    let ptr = unsafe { buffer.as_ptr().add(1) };
    let result = process_quickslot_add(ptr);

    assert!(result);

    let res = m2_quickslot_get_v2(5);
    assert_eq!(res.slot_type, SLOT_TYPE_SKILL);
    assert_eq!(res.slot_position, 12);
}
