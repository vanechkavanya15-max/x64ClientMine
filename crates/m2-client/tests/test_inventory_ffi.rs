use m2_client::inventory_ffi::{
    m2_inv_get_item, m2_inv_move_item, m2_inv_use_item, m2_inv_get_gold,
    set_item_for_test, reset_inventory_for_test, set_gold_for_test,
};

use std::sync::Mutex;

static TEST_MUTEX: Mutex<()> = Mutex::new(());

#[test]
fn test_m2_inv_get_item() {
    let _guard = TEST_MUTEX.lock().unwrap();
    reset_inventory_for_test();
    set_item_for_test(1, 10, 5001, 5);

    let mut vnum: u32 = 0;
    let mut count: u8 = 0;

    // Pobranie istniejacego przedmiotu
    let result = m2_inv_get_item(1, 10, &mut vnum, &mut count);
    assert!(result);
    assert_eq!(vnum, 5001);
    assert_eq!(count, 5);

    // Pobranie nieistniejacego przedmiotu
    let result_empty = m2_inv_get_item(1, 99, &mut vnum, &mut count);
    assert!(!result_empty);

    // Test pustych wskaznikow (null pointer)
    let null_vnum_result = m2_inv_get_item(1, 10, std::ptr::null_mut(), &mut count);
    assert!(!null_vnum_result);

    let null_count_result = m2_inv_get_item(1, 10, &mut vnum, std::ptr::null_mut());
    assert!(!null_count_result);
}

#[test]
fn test_m2_inv_move_item() {
    let _guard = TEST_MUTEX.lock().unwrap();
    reset_inventory_for_test();
    set_item_for_test(1, 0, 1001, 10);
    
    // Przeniesienie calego stosu do pustego slota
    let move_result = m2_inv_move_item(1, 0, 1, 1, 10);
    assert!(move_result);
    
    let mut vnum: u32 = 0;
    let mut count: u8 = 0;
    let exists = m2_inv_get_item(1, 1, &mut vnum, &mut count);
    assert!(exists);
    assert_eq!(vnum, 1001);
    assert_eq!(count, 10);
    assert!(!m2_inv_get_item(1, 0, &mut vnum, &mut count));

    // Przeniesienie czesci stosu do pustego slota
    let move_partial = m2_inv_move_item(1, 1, 1, 2, 3);
    assert!(move_partial);
    assert!(m2_inv_get_item(1, 1, &mut vnum, &mut count));
    assert_eq!(count, 7);
    assert!(m2_inv_get_item(1, 2, &mut vnum, &mut count));
    assert_eq!(count, 3);

    // Polaczenie dwoch stosow (tego samego typu)
    let move_merge = m2_inv_move_item(1, 2, 1, 1, 3);
    assert!(move_merge);
    assert!(m2_inv_get_item(1, 1, &mut vnum, &mut count));
    assert_eq!(count, 10);
    assert!(!m2_inv_get_item(1, 2, &mut vnum, &mut count));

    // Zamiana przedmiotow miejscami (inny typ, caly stos)
    set_item_for_test(1, 3, 2002, 5);
    let move_swap = m2_inv_move_item(1, 1, 1, 3, 10);
    assert!(move_swap);
    
    assert!(m2_inv_get_item(1, 1, &mut vnum, &mut count));
    assert_eq!(vnum, 2002);
    assert_eq!(count, 5);

    assert!(m2_inv_get_item(1, 3, &mut vnum, &mut count));
    assert_eq!(vnum, 1001);
    assert_eq!(count, 10);

    // Nieudane przeniesienie z powodu mniejszej ilosci w stosie niz zadana
    let move_fail_count = m2_inv_move_item(1, 1, 1, 4, 100);
    assert!(!move_fail_count);

    // Przeniesienie w to samo miejsce
    let move_fail_same = m2_inv_move_item(1, 1, 1, 1, 1);
    assert!(!move_fail_same);

    // Przeniesienie zerowej ilosci
    let move_fail_zero = m2_inv_move_item(1, 1, 1, 5, 0);
    assert!(!move_fail_zero);
}

#[test]
fn test_m2_inv_use_item() {
    let _guard = TEST_MUTEX.lock().unwrap();
    reset_inventory_for_test();
    set_item_for_test(1, 5, 3003, 2);

    // Uzycie przedmiotu (zmniejszenie licznika)
    let use_result = m2_inv_use_item(1, 5);
    assert!(use_result);

    let mut vnum: u32 = 0;
    let mut count: u8 = 0;
    assert!(m2_inv_get_item(1, 5, &mut vnum, &mut count));
    assert_eq!(count, 1);

    // Uzycie ostatniego przedmiotu z grupy (usuniecie)
    let use_last = m2_inv_use_item(1, 5);
    assert!(use_last);
    assert!(!m2_inv_get_item(1, 5, &mut vnum, &mut count));

    // Uzycie pustego slota
    let use_empty = m2_inv_use_item(1, 5);
    assert!(!use_empty);
}

#[test]
fn test_m2_inv_get_gold() {
    let _guard = TEST_MUTEX.lock().unwrap();
    reset_inventory_for_test();
    set_gold_for_test(150_000_000);

    let gold = m2_inv_get_gold();
    assert_eq!(gold, 150_000_000);

    set_gold_for_test(0);
    assert_eq!(m2_inv_get_gold(), 0);
}
