use m2_client::ui_window_inventory::{
    UiWindowInventory,
    
    get_ui_window_inventory,
};


#[test]
fn test_window_composition() {
    let inventory = UiWindowInventory::default();
    assert_eq!(inventory.base.width, 176);
    assert_eq!(inventory.base.height, 375);
    assert_eq!(inventory.base.name, "InventoryWindow");
    assert_eq!(inventory.is_visible, false);
}

#[test]
fn test_yang_formatting_with_separators() {
    assert_eq!(UiWindowInventory::format_yang(0), "0 Yang");
    assert_eq!(UiWindowInventory::format_yang(500), "500 Yang");
    assert_eq!(UiWindowInventory::format_yang(1000), "1,000 Yang");
    assert_eq!(UiWindowInventory::format_yang(1500000), "1,500,000 Yang");
    assert_eq!(UiWindowInventory::format_yang(1234567890), "1,234,567,890 Yang");
    assert_eq!(UiWindowInventory::format_yang(-1500000), "-1,500,000 Yang");
}

#[test]
fn test_visibility_toggle_hotkey() {
    let mut inventory = UiWindowInventory::default();
    
    // Default
    assert_eq!(inventory.is_visible, false);
    assert_eq!(inventory.base.is_show, false);
    
    // Toggle to show
    inventory.toggle_visibility();
    assert_eq!(inventory.is_visible, true);
    assert_eq!(inventory.base.is_show, true);
    
    // Toggle to hide
    inventory.toggle_visibility();
    assert_eq!(inventory.is_visible, false);
    assert_eq!(inventory.base.is_show, false);
}

#[test]
fn test_inventory_page_switching() {
    let mut inventory = UiWindowInventory::default();
    
    // Default page is 0
    assert_eq!(inventory.active_page, 0);
    
    // Change to valid page (2)
    inventory.set_active_page(2);
    assert_eq!(inventory.active_page, 2);
    
    // Change to valid page (3)
    inventory.set_active_page(3);
    assert_eq!(inventory.active_page, 3);
    
    // Change to invalid page (4) - should be ignored
    inventory.set_active_page(4);
    assert_eq!(inventory.active_page, 3);
}

#[test]
fn test_grid_slot_index() {
    let inventory = UiWindowInventory::default();
    
    // Page 0 bounds
    assert_eq!(inventory.get_global_slot_index(0, 0), Some(0));
    assert_eq!(inventory.get_global_slot_index(0, 44), Some(44));
    assert_eq!(inventory.get_global_slot_index(0, 45), None); // Out of bounds local
    
    // Page 1 bounds
    assert_eq!(inventory.get_global_slot_index(1, 0), Some(45));
    assert_eq!(inventory.get_global_slot_index(1, 44), Some(89));
    
    // Page 2 bounds
    assert_eq!(inventory.get_global_slot_index(2, 0), Some(90));
    assert_eq!(inventory.get_global_slot_index(2, 44), Some(134));
    
    // Page 3 bounds
    assert_eq!(inventory.get_global_slot_index(3, 0), Some(135));
    assert_eq!(inventory.get_global_slot_index(3, 44), Some(179));
    
    // Invalid Page
    assert_eq!(inventory.get_global_slot_index(4, 0), None);
}

#[test]
fn test_singleton_instance() {
    let mutex = get_ui_window_inventory();
    
    {
        let mut inv = mutex.lock().unwrap();
        inv.set_active_page(1);
    } // lock dropped

    {
        let inv = mutex.lock().unwrap();
        assert_eq!(inv.active_page, 1);
    }
}

#[test]
fn test_hotkey_handler() {
    let mut inventory = UiWindowInventory::default();
    assert_eq!(inventory.is_visible, false);
    
    // Press 'I'
    inventory.handle_key_press('I');
    assert_eq!(inventory.is_visible, true);
    
    // Press 'i'
    inventory.handle_key_press('i');
    assert_eq!(inventory.is_visible, false);
    
    // Ignore other keys
    inventory.handle_key_press('A');
    assert_eq!(inventory.is_visible, false);
}

use m2_client::item_anti_flags::{PlayerClass, PlayerSex, Item as AntiFlagItem};

#[test]
fn test_player_state_synchronization() {
    let inventory = UiWindowInventory::default();
    
    // Create a level 10 warrior male item
    let warrior_item = AntiFlagItem {
        anti_flags: m2_client::item_anti_flags::ITEM_ANTIFLAG_ASSASSIN | m2_client::item_anti_flags::ITEM_ANTIFLAG_SURA | m2_client::item_anti_flags::ITEM_ANTIFLAG_SHAMAN,
        limit_level: 10,
    };
    
    // Level 10 Warrior can equip
    assert!(inventory.can_equip_item_to_player(&warrior_item, PlayerClass::Warrior, PlayerSex::Male, 10));
    
    // Level 9 Warrior cannot equip
    assert!(!inventory.can_equip_item_to_player(&warrior_item, PlayerClass::Warrior, PlayerSex::Male, 9));
    
    // Level 10 Assassin cannot equip
    assert!(!inventory.can_equip_item_to_player(&warrior_item, PlayerClass::Assassin, PlayerSex::Male, 10));
}
