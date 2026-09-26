#[path = "../src/safebox_inventory.rs"]
mod safebox_inventory;

use safebox_inventory::{SafeBox, ItemData, SAFEBOX_PAGE_SIZE};

#[test]
fn test_safebox_initialization() {
    let safebox = SafeBox::new(2);
    assert_eq!(safebox.get_size(), SAFEBOX_PAGE_SIZE * 2);
    assert_eq!(safebox.get_gold(), 0);
}

#[test]
fn test_safebox_initialization_bounds() {
    // 0 pages should initialize to 1 page
    let safebox_zero = SafeBox::new(0);
    assert_eq!(safebox_zero.get_size(), SAFEBOX_PAGE_SIZE);

    // Over 3 pages should clamp to 3 pages
    let safebox_over = SafeBox::new(5);
    assert_eq!(safebox_over.get_size(), SAFEBOX_PAGE_SIZE * 3);
}

#[test]
fn test_deposit_gold() {
    let mut safebox = SafeBox::new(1);
    assert!(safebox.deposit_gold(100));
    assert_eq!(safebox.get_gold(), 100);

    // Cannot deposit negative gold
    assert!(!safebox.deposit_gold(-50));
    assert_eq!(safebox.get_gold(), 100);

    // Cannot deposit 0 gold
    assert!(!safebox.deposit_gold(0));
    assert_eq!(safebox.get_gold(), 100);

    // Deposit overflow check
    assert!(!safebox.deposit_gold(i64::MAX));
    assert_eq!(safebox.get_gold(), 100);
}

#[test]
fn test_withdraw_gold() {
    let mut safebox = SafeBox::new(1);
    safebox.deposit_gold(500);

    assert!(safebox.withdraw_gold(200));
    assert_eq!(safebox.get_gold(), 300);

    // Cannot withdraw negative gold
    assert!(!safebox.withdraw_gold(-100));
    assert_eq!(safebox.get_gold(), 300);

    // Cannot withdraw 0 gold
    assert!(!safebox.withdraw_gold(0));
    assert_eq!(safebox.get_gold(), 300);

    // Cannot withdraw more than balance
    assert!(!safebox.withdraw_gold(500));
    assert_eq!(safebox.get_gold(), 300);

    // Withdraw exact remaining balance
    assert!(safebox.withdraw_gold(300));
    assert_eq!(safebox.get_gold(), 0);
}

#[test]
fn test_check_in_and_check_out() {
    let mut safebox = SafeBox::new(1);
    let item = ItemData {
        vnum: 1234,
        count: 1,
        flags: 0,
        anti_flags: 0,
        al_sockets: [0; 3],
        a_attr: [Default::default(); 7],
    };

    // Valid check_in
    assert!(safebox.check_in(0, item));
    
    // Slot now occupied
    assert!(!safebox.check_in(0, item));

    // Cannot check in empty item
    let empty_item = ItemData::default();
    assert!(!safebox.check_in(1, empty_item));

    // Cannot check in to out of bounds slot
    assert!(!safebox.check_in(SAFEBOX_PAGE_SIZE, item));

    // Valid check_out
    let checked_out_item = safebox.check_out(0);
    assert!(checked_out_item.is_some());
    assert_eq!(checked_out_item.unwrap().vnum, 1234);

    // Slot is now empty, cannot check out again
    assert!(safebox.check_out(0).is_none());

    // Cannot check out from out of bounds slot
    assert!(safebox.check_out(SAFEBOX_PAGE_SIZE).is_none());
}

#[test]
fn test_get_item() {
    let mut safebox = SafeBox::new(1);
    let item = ItemData {
        vnum: 4321,
        count: 5,
        ..Default::default()
    };
    
    safebox.check_in(5, item);
    
    let retrieved = safebox.get_item(5);
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().vnum, 4321);
    assert_eq!(retrieved.unwrap().count, 5);

    // Empty slot
    let empty_slot = safebox.get_item(6);
    assert!(empty_slot.is_some());
    assert_eq!(empty_slot.unwrap().vnum, 0);

    // Out of bounds
    assert!(safebox.get_item(SAFEBOX_PAGE_SIZE).is_none());
}
