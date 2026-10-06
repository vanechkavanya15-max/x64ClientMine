#[path = "../src/safebox_storage_manager_v2.rs"]
mod safebox_storage_manager_v2;

use safebox_storage_manager_v2::{CPythonSafeBox, TItemData};

#[test]
fn test_safebox_open_and_size() {
    let mut safebox = CPythonSafeBox::new();
    assert_eq!(safebox.GetCurrentSafeBoxSize(), 0);

    safebox.OpenSafeBox(1);
    assert_eq!(safebox.GetCurrentSafeBoxSize(), 45); // 5 * 9 * 1

    safebox.OpenSafeBox(3);
    assert_eq!(safebox.GetCurrentSafeBoxSize(), 135); // 5 * 9 * 3
}

#[test]
fn test_safebox_item_operations() {
    let mut safebox = CPythonSafeBox::new();
    safebox.OpenSafeBox(1);

    let mut item = TItemData::default();
    item.vnum = 1001;
    item.count = 5;

    safebox.SetItemData(0, item.clone());
    safebox.SetItemData(44, item.clone());

    // Verify getting item ID
    let mut item_id = 0;
    assert!(safebox.GetSlotItemID(0, &mut item_id));
    assert_eq!(item_id, 1001);

    // Verify getting item pointer data
    let ptr = safebox.GetItemDataPtr(0).unwrap();
    assert_eq!(ptr.vnum, 1001);
    assert_eq!(ptr.count, 5);

    // Delete item
    safebox.DelItemData(0);
    assert!(safebox.GetSlotItemID(0, &mut item_id));
    assert_eq!(item_id, 0);

    // Out of bounds checks
    assert!(!safebox.GetSlotItemID(100, &mut item_id));
    assert!(safebox.GetItemDataPtr(100).is_none());
}

#[test]
fn test_mall_operations() {
    let mut safebox = CPythonSafeBox::new();
    assert_eq!(safebox.GetMallSize(), 0);

    safebox.OpenMall(2);
    assert_eq!(safebox.GetMallSize(), 90);

    let mut item = TItemData::default();
    item.vnum = 2002;
    safebox.SetMallItemData(10, item);

    let mut item_id = 0;
    assert!(safebox.GetSlotMallItemID(10, &mut item_id));
    assert_eq!(item_id, 2002);

    let ptr = safebox.GetMallItemDataPtr(10).unwrap();
    assert_eq!(ptr.vnum, 2002);

    safebox.DelMallItemData(10);
    assert!(safebox.GetSlotMallItemID(10, &mut item_id));
    assert_eq!(item_id, 0);

    assert!(!safebox.GetSlotMallItemID(200, &mut item_id));
    assert!(safebox.GetMallItemDataPtr(200).is_none());
}

#[test]
fn test_money_operations() {
    let mut safebox = CPythonSafeBox::new();
    assert_eq!(safebox.GetMoney(), 0);

    safebox.SetMoney(5000);
    assert_eq!(safebox.GetMoney(), 5000);

    assert!(safebox.deposit_yang(2000));
    assert_eq!(safebox.GetMoney(), 7000);

    assert!(safebox.withdraw_yang(3000));
    assert_eq!(safebox.GetMoney(), 4000);

    assert!(!safebox.withdraw_yang(5000));
    assert_eq!(safebox.GetMoney(), 4000);

    assert!(!safebox.deposit_yang(0));
    assert!(!safebox.withdraw_yang(0));
}

#[test]
fn test_password_operations() {
    let mut safebox = CPythonSafeBox::new();
    
    // Default is 000000
    assert!(safebox.verify_password("000000"));
    assert!(!safebox.verify_password("123456"));

    // Change expected hash
    safebox.set_expected_password("abcdef");
    assert!(!safebox.verify_password("000000"));
    assert!(safebox.verify_password("abcdef"));
}
