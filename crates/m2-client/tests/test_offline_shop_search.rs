use m2_client::game_offline_shop_search_engine::*;
use std::ffi::CString;

#[test]
fn test_offline_shop_search_and_sorting() {
    m2_offline_shop_clear();

    let owner1 = CString::new("PlayerA").unwrap();
    let title1 = CString::new("Shop Cheap").unwrap();
    m2_offline_shop_register_item(1001, owner1.as_ptr(), title1.as_ptr(), 10, 1, 5000);

    let owner2 = CString::new("PlayerB").unwrap();
    let title2 = CString::new("Shop Expensive").unwrap();
    m2_offline_shop_register_item(1002, owner2.as_ptr(), title2.as_ptr(), 10, 1, 2000);

    let count = m2_offline_shop_search_vnum(10);
    assert_eq!(count, 2);

    // Sorted by price asc: 2000 first (PlayerB), then 5000 (PlayerA)
    let mut vid = 0u32;
    let mut price = 0u64;

    assert!(m2_offline_shop_get_result(0, &mut vid, &mut price));
    assert_eq!(vid, 1002);
    assert_eq!(price, 2000);

    assert!(m2_offline_shop_get_result(1, &mut vid, &mut price));
    assert_eq!(vid, 1001);
    assert_eq!(price, 5000);
}
