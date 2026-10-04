use m2_client::game_safebox_storage_manager::{GameSafeboxStorageManager, TItemData};

#[test]
fn test_password_verification() {
    let mut manager = GameSafeboxStorageManager::new();
    
    // Default dummy password is "000000"
    assert!(!manager.is_open());
    
    assert!(!manager.checkin_password("123456"));
    assert!(!manager.is_open());
    
    assert!(manager.checkin_password("000000"));
    assert!(manager.is_open());
    
    manager.set_expected_password("mypassword");
    assert!(!manager.checkin_password("wrong"));
    assert!(manager.checkin_password("mypassword"));
}

#[test]
fn test_yang_deposit_withdraw() {
    let mut manager = GameSafeboxStorageManager::new();
    manager.open_safebox(1);
    
    assert_eq!(manager.get_money(), 0);
    
    // Deposit
    assert!(manager.deposit_yang(1000));
    assert_eq!(manager.get_money(), 1000);
    
    assert!(manager.deposit_yang(500));
    assert_eq!(manager.get_money(), 1500);
    
    // Zero deposit
    assert!(!manager.deposit_yang(0));
    
    // Withdraw
    assert!(manager.withdraw_yang(200));
    assert_eq!(manager.get_money(), 1300);
    
    // Over withdraw
    assert!(!manager.withdraw_yang(2000));
    assert_eq!(manager.get_money(), 1300);
    
    // Exact withdraw
    assert!(manager.withdraw_yang(1300));
    assert_eq!(manager.get_money(), 0);
    
    // Set explicitly
    manager.set_money(5000);
    assert_eq!(manager.get_money(), 5000);
}

#[test]
fn test_item_checkin_checkout() {
    let mut manager = GameSafeboxStorageManager::new();
    manager.open_safebox(1); // 45 slots
    
    assert_eq!(manager.get_current_safebox_size(), 45);
    
    let mut item1 = TItemData::default();
    item1.vnum = 11299;
    item1.count = 1;
    
    // Check in valid item
    assert!(manager.checkin_item(0, item1.clone()));
    
    // Check in same slot should fail
    let mut item2 = TItemData::default();
    item2.vnum = 299;
    assert!(!manager.checkin_item(0, item2.clone()));
    
    // Check out
    let checked_out = manager.checkout_item(0);
    assert!(checked_out.is_some());
    assert_eq!(checked_out.unwrap().vnum, 11299);
    
    // Check out empty slot
    assert!(manager.checkout_item(0).is_none());
    
    // Check in empty item
    assert!(!manager.checkin_item(1, TItemData::default()));
    
    // Check out of bounds
    assert!(!manager.checkin_item(50, item1.clone()));
    assert!(manager.checkout_item(50).is_none());
}

#[test]
fn test_safebox_pages_and_mall() {
    let mut manager = GameSafeboxStorageManager::new();
    
    manager.open_safebox(3);
    assert_eq!(manager.get_current_safebox_size(), 3 * 45); // 135
    
    manager.open_mall(1);
    assert_eq!(manager.get_mall_size(), 45);
    
    let mut item = TItemData::default();
    item.vnum = 5000;
    
    manager.set_mall_item_data(5, item.clone());
    
    let mall_item = manager.get_mall_item_data(5);
    assert!(mall_item.is_some());
    assert_eq!(mall_item.unwrap().vnum, 5000);
    
    manager.del_mall_item_data(5);
    assert_eq!(manager.get_mall_item_data(5).unwrap().vnum, 0);
}
