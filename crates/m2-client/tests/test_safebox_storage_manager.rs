use m2_client::safebox_storage_manager::{SafeboxStorageManager, TItemData, TPlayerItemAttribute};

#[test]
fn test_safebox_initialization_sizes() {
    let mut manager = SafeboxStorageManager::new();
    
    // Test size 1 (45 slots)
    manager.open_safebox(1);
    assert_eq!(manager.get_current_safebox_size(), 45);

    // Test size 2 (90 slots)
    manager.open_safebox(2);
    assert_eq!(manager.get_current_safebox_size(), 90);

    // Test size 3 (135 slots)
    manager.open_safebox(3);
    assert_eq!(manager.get_current_safebox_size(), 135);

    // Test max boundary (should be clamped to 3)
    manager.open_safebox(4);
    assert_eq!(manager.get_current_safebox_size(), 135);

    // Test min boundary (should be clamped to 1)
    manager.open_safebox(0);
    assert_eq!(manager.get_current_safebox_size(), 45);
}

#[test]
fn test_safebox_item_operations() {
    let mut manager = SafeboxStorageManager::new();
    manager.open_safebox(1);

    let item = TItemData {
        vnum: 1234,
        count: 1,
        flags: 0,
        anti_flags: 0,
        alSockets: [0; 3],
        aAttr: [TPlayerItemAttribute::default(); 7],
    };

    // Test setting item
    manager.set_item_data(5, item.clone());
    let fetched = manager.get_item_data(5).unwrap();
    assert_eq!(fetched.vnum, 1234);

    // Test out of bounds set (should safely ignore)
    manager.set_item_data(100, item.clone());

    // Test deleting item
    manager.del_item_data(5);
    let fetched = manager.get_item_data(5).unwrap();
    assert_eq!(fetched.vnum, 0); // Default vnum
}

#[test]
fn test_safebox_move_operations() {
    let mut manager = SafeboxStorageManager::new();
    manager.open_safebox(1);

    let item = TItemData {
        vnum: 999,
        count: 5,
        flags: 0,
        anti_flags: 0,
        alSockets: [0; 3],
        aAttr: [TPlayerItemAttribute::default(); 7],
    };

    // Test checkin
    assert!(manager.checkin_item(10, item.clone()));
    assert_eq!(manager.get_item_data(10).unwrap().vnum, 999);
    
    // Checkin to occupied slot
    assert!(!manager.checkin_item(10, item.clone()));

    // Test move
    assert!(manager.move_item(10, 11));
    assert_eq!(manager.get_item_data(10).unwrap().vnum, 0);
    assert_eq!(manager.get_item_data(11).unwrap().vnum, 999);

    // Test checkout
    let checked_out = manager.checkout_item(11).unwrap();
    assert_eq!(checked_out.vnum, 999);
    assert_eq!(manager.get_item_data(11).unwrap().vnum, 0);
}

#[test]
fn test_mall_initialization_and_operations() {
    let mut manager = SafeboxStorageManager::new();
    
    // Mall size is clamped max to 3 like safebox, but generally 1 is standard
    manager.open_mall(1);
    assert_eq!(manager.get_mall_size(), 45);

    let item = TItemData {
        vnum: 5555,
        count: 10,
        flags: 0,
        anti_flags: 0,
        alSockets: [0; 3],
        aAttr: [TPlayerItemAttribute::default(); 7],
    };

    manager.set_mall_item_data(0, item.clone());
    assert_eq!(manager.get_mall_item_data(0).unwrap().vnum, 5555);

    manager.del_mall_item_data(0);
    assert_eq!(manager.get_mall_item_data(0).unwrap().vnum, 0);
}

#[test]
fn test_money_operations() {
    let mut manager = SafeboxStorageManager::new();
    manager.set_money(1000);
    assert_eq!(manager.get_money(), 1000);

    // Test deposit
    assert!(manager.deposit_yang(500));
    assert_eq!(manager.get_money(), 1500);

    // Test withdraw
    assert!(manager.withdraw_yang(300));
    assert_eq!(manager.get_money(), 1200);

    // Test withdraw too much
    assert!(!manager.withdraw_yang(5000));
    assert_eq!(manager.get_money(), 1200);

    // Test invalid negative amounts (should fail/return false)
    assert!(!manager.deposit_yang(-100));
    assert!(!manager.withdraw_yang(-100));
}

#[test]
fn test_password_states() {
    let mut manager = SafeboxStorageManager::new();
    manager.set_expected_password("123456");

    assert!(!manager.is_open());

    // Wrong password
    assert!(!manager.checkin_password("wrong"));
    assert!(!manager.is_open());

    // Correct password
    assert!(manager.checkin_password("123456"));
    assert!(manager.is_open());
}
