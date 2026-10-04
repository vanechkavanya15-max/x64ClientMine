// Testy menedzera paska szybkiego dostepu (QuickSlot Bar) dla Metin2.
use m2_client::ui_window_quickslot_bar::{QuickSlotBarManager, SLOT_TYPE_ITEM, SLOT_TYPE_SKILL, QUICKSLOT_MAX_NUM_SERVER};

#[test]
fn test_quickslot_initialization() {
    let manager = QuickSlotBarManager::new();
    
    assert_eq!(manager.active_page, 0);
    assert_eq!(manager.slots.len(), QUICKSLOT_MAX_NUM_SERVER);
    
    for slot in manager.slots.iter() {
        assert_eq!(slot.slot_type, 0);
        let pos = slot.slot_position;
        assert_eq!(pos, 0);
    }
}

#[test]
fn test_set_page_and_global_index() {
    let mut manager = QuickSlotBarManager::new();
    
    // Default page
    assert_eq!(manager.active_page, 0);
    assert_eq!(manager.get_global_index(0), 0);
    assert_eq!(manager.get_global_index(7), 7);
    
    // Page 1
    manager.set_page(1);
    assert_eq!(manager.active_page, 1);
    assert_eq!(manager.get_global_index(0), 8);
    assert_eq!(manager.get_global_index(7), 15);
    
    // Page 3 (max valid index is 3)
    manager.set_page(3);
    assert_eq!(manager.active_page, 3);
    assert_eq!(manager.get_global_index(0), 24);
    assert_eq!(manager.get_global_index(7), 31);
    
    // Wrap around forward
    manager.set_page(4);
    assert_eq!(manager.active_page, 0);
    
    // Wrap around backward
    manager.set_page(-1);
    assert_eq!(manager.active_page, 3);
    
    manager.set_page(-5);
    assert_eq!(manager.active_page, 3);
}

#[test]
fn test_add_and_use_quickslot() {
    let mut manager = QuickSlotBarManager::new();
    manager.set_page(0);
    
    // Add item to local pos 0 (key '1')
    manager.add_quickslot(0, SLOT_TYPE_ITEM, 150);
    
    assert_eq!(manager.slots[0].slot_type, SLOT_TYPE_ITEM);
    let pos0 = manager.slots[0].slot_position;
    assert_eq!(pos0, 150);
    
    // Use via key code 0x31 ('1') without shift
    let result = manager.on_key_pressed(0x31, false);
    assert!(result.is_some());
    let used = result.unwrap();
    assert_eq!(used.slot_type, SLOT_TYPE_ITEM);
    let used_pos = used.slot_position;
    assert_eq!(used_pos, 150);
    
    // Try using empty slot
    let empty_result = manager.on_key_pressed(0x32, false);
    assert!(empty_result.is_none());
    
    // Switch page using Shift+1
    let shift_result = manager.on_key_pressed(0x31, true);
    assert!(shift_result.is_none());
    assert_eq!(manager.active_page, 0);
    
    // Switch to page 2 (index 1) using Shift+2
    let shift2_result = manager.on_key_pressed(0x32, true);
    assert!(shift2_result.is_none());
    assert_eq!(manager.active_page, 1);
}

#[test]
fn test_remove_and_swap_quickslot() {
    let mut manager = QuickSlotBarManager::new();
    manager.set_page(0);
    
    // Add item and skill
    manager.add_quickslot(0, SLOT_TYPE_ITEM, 100);
    manager.add_quickslot(1, SLOT_TYPE_SKILL, 15);
    
    // Swap
    manager.swap_quickslot(0, 1);
    
    assert_eq!(manager.slots[0].slot_type, SLOT_TYPE_SKILL);
    let p0 = manager.slots[0].slot_position;
    assert_eq!(p0, 15);
    
    assert_eq!(manager.slots[1].slot_type, SLOT_TYPE_ITEM);
    let p1 = manager.slots[1].slot_position;
    assert_eq!(p1, 100);
    
    // Remove
    manager.remove_quickslot(0);
    assert_eq!(manager.slots[0].slot_type, 0);
    let p0_after = manager.slots[0].slot_position;
    assert_eq!(p0_after, 0);
    
    // F1 test (key 0x70) which maps to local index 4
    manager.add_quickslot(4, SLOT_TYPE_SKILL, 99);
    let f1_result = manager.on_key_pressed(0x70, false);
    assert!(f1_result.is_some());
    let f1_used = f1_result.unwrap();
    assert_eq!(f1_used.slot_type, SLOT_TYPE_SKILL);
    let f1_pos = f1_used.slot_position;
    assert_eq!(f1_pos, 99);
}
