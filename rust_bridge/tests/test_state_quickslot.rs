use m2_client_bridge::state::quickslot::*;
use std::ptr;

#[test]
fn test_quickslot_manager_new() {
    let mgr = QuickslotManager::new();
    for i in 0..32 {
        let entry = mgr.get_slot(i);
        assert_eq!(entry.slot_type, SLOT_TYPE_NONE);
        assert_eq!({entry.pos}, 0);
    }
}

#[test]
fn test_quickslot_set_and_get() {
    let mut mgr = QuickslotManager::new();
    mgr.set_slot(0, SLOT_TYPE_SKILL, 10);
    mgr.set_slot(31, SLOT_TYPE_ITEM, 5);

    let entry0 = mgr.get_slot(0);
    assert_eq!(entry0.slot_type, SLOT_TYPE_SKILL);
    assert_eq!({entry0.pos}, 10);

    let entry31 = mgr.get_slot(31);
    assert_eq!(entry31.slot_type, SLOT_TYPE_ITEM);
    assert_eq!({entry31.pos}, 5);
}

#[test]
fn test_quickslot_clear() {
    let mut mgr = QuickslotManager::new();
    mgr.set_slot(5, SLOT_TYPE_ITEM, 2);
    assert_eq!(mgr.get_slot(5).slot_type, SLOT_TYPE_ITEM);

    mgr.clear_slot(5);
    let entry = mgr.get_slot(5);
    assert_eq!(entry.slot_type, SLOT_TYPE_NONE);
    assert_eq!({entry.pos}, 0);
}

#[test]
fn test_quickslot_swap() {
    let mut mgr = QuickslotManager::new();
    mgr.set_slot(1, SLOT_TYPE_SKILL, 100);
    mgr.set_slot(2, SLOT_TYPE_ITEM, 50);

    mgr.swap_slots(1, 2);

    let entry1 = mgr.get_slot(1);
    assert_eq!(entry1.slot_type, SLOT_TYPE_ITEM);
    assert_eq!({entry1.pos}, 50);

    let entry2 = mgr.get_slot(2);
    assert_eq!(entry2.slot_type, SLOT_TYPE_SKILL);
    assert_eq!({entry2.pos}, 100);
}

#[test]
fn test_ffi_m2_quickslot_get() {
    unsafe {
        // Modyfikujemy stan globalny na potrzeby testu FFI
        m2_quickslot_set(4, SLOT_TYPE_SKILL, 42);

        let mut slot_type: u8 = 0;
        let mut pos: u32 = 0;

        let result = m2_quickslot_get(4, &mut slot_type, &mut pos);
        assert!(result);
        assert_eq!(slot_type, SLOT_TYPE_SKILL);
        assert_eq!(pos, 42);

        // Test dla inwalidnego indeksu
        let result_invalid = m2_quickslot_get(32, &mut slot_type, &mut pos);
        assert!(!result_invalid);
    }
}

#[test]
fn test_ffi_memory_management() {
    unsafe {
        let ptr = m2_quickslot_manager_new();
        assert!(!ptr.is_null());
        m2_quickslot_manager_free(ptr);
    }
}
