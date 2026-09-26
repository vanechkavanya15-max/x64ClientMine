use m2_client::safebox_mall_manager_v2::*;
use std::ffi::CString;

#[test]
fn test_safebox_initialization_and_size() {
    let mut manager = SafeboxMallManagerV2::new("12345");
    assert!(!manager.is_open());
    assert_eq!(manager.get_wrong_attempts(), 0);
    assert_eq!(manager.get_current_safebox_size(), 0);
    
    // Safebox opens with size 3 (meaning 3 * SAFEBOX_SLOT_X_COUNT items space but let's test how C++ does it)
    // Actually C++ does: m_ItemInstanceVector.resize(SAFEBOX_SLOT_X_COUNT * iSize);
    // Which means if iSize = 27 (for 3 pages of 9 rows), it is 5 * 27 = 135
    manager.open_safebox(27);
    assert_eq!(manager.get_current_safebox_size(), 135);
    assert_eq!(manager.get_money(), 0);
}

#[test]
fn test_safebox_item_operations() {
    let mut manager = SafeboxMallManagerV2::new("12345");
    manager.open_safebox(27);
    
    let mut item = TItemData::default();
    item.vnum = 1001;
    item.count = 5;
    
    manager.set_item_data(10, item.clone());
    
    let retrieved = manager.get_item_data(10).unwrap();
    assert_eq!(retrieved.vnum, 1001);
    assert_eq!(retrieved.count, 5);
    
    // Invalid index should not panic
    manager.set_item_data(1000, item.clone());
    assert!(manager.get_item_data(1000).is_none());
    
    manager.del_item_data(10);
    let deleted = manager.get_item_data(10).unwrap();
    assert_eq!(deleted.vnum, 0);
    assert_eq!(deleted.count, 0);
}

#[test]
fn test_mall_item_operations() {
    let mut manager = SafeboxMallManagerV2::new("12345");
    manager.open_mall(9); // 1 page (5 * 9 = 45 slots)
    assert_eq!(manager.get_mall_size(), 45);
    
    let mut item = TItemData::default();
    item.vnum = 2002;
    item.count = 1;
    
    manager.set_mall_item_data(5, item.clone());
    
    let retrieved = manager.get_mall_item_data(5).unwrap();
    assert_eq!(retrieved.vnum, 2002);
    
    manager.del_mall_item_data(5);
    let deleted = manager.get_mall_item_data(5).unwrap();
    assert_eq!(deleted.vnum, 0);
}

#[test]
fn test_password_logic() {
    let mut manager = SafeboxMallManagerV2::new("secret");
    
    assert!(!manager.try_open_with_password("wrong"));
    assert_eq!(manager.get_wrong_attempts(), 1);
    assert!(!manager.is_open());
    
    assert!(!manager.try_open_with_password("wrong2"));
    assert_eq!(manager.get_wrong_attempts(), 2);
    
    assert!(manager.try_open_with_password("secret"));
    assert!(manager.is_open());
    assert_eq!(manager.get_wrong_attempts(), 0);
}

#[test]
fn test_money_logic() {
    let mut manager = SafeboxMallManagerV2::new("secret");
    manager.open_safebox(27);
    
    assert_eq!(manager.get_money(), 0);
    manager.set_money(5000);
    assert_eq!(manager.get_money(), 5000);
}

#[test]
fn test_ffi_m2_safebox_open() {
    // Reset global state by accessing it safely
    {
        let mut manager = G_SAFEBOX_MALL_MANAGER.write().unwrap();
        manager.set_expected_password("111111");
        // simulate locked state
        manager.try_open_with_password("wrong");
    }
    
    let correct_pass = CString::new("111111").unwrap();
    let wrong_pass = CString::new("000000").unwrap();
    
    let res = m2_safebox_open(wrong_pass.as_ptr() as *const u8);
    assert!(!res);
    
    let res2 = m2_safebox_open(correct_pass.as_ptr() as *const u8);
    assert!(res2);
    
    let null_res = m2_safebox_open(std::ptr::null());
    assert!(!null_res);
}
