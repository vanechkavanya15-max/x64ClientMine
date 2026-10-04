use m2_client::py_bridge_shop_tabs_multi::*;
use std::ffi::CString;

#[test]
fn test_shop_multi_tabs() {
    m2_py_shop_set_tab_count(3);
    assert_eq!(m2_py_shop_get_tab_count(), 3);

    let name0 = CString::new("Weapons").unwrap();
    let name1 = CString::new("Armors").unwrap();
    let name2 = CString::new("Upgrades").unwrap();

    m2_py_shop_set_tab_name(0, name0.as_ptr());
    m2_py_shop_set_tab_name(1, name1.as_ptr());
    m2_py_shop_set_tab_name(2, name2.as_ptr());

    m2_py_shop_set_tab_coin_type(0, 0); // Yang
    m2_py_shop_set_tab_coin_type(1, 0); // Yang
    m2_py_shop_set_tab_coin_type(2, 1); // Won/Cheque

    assert_eq!(m2_py_shop_get_tab_coin_type(0), 0);
    assert_eq!(m2_py_shop_get_tab_coin_type(2), 1);

    let mut buf = [0u8; 32];
    assert!(m2_py_shop_get_tab_name(0, buf.as_mut_ptr() as *mut _, buf.len()));
    let read_str = std::str::from_utf8(&buf).unwrap().trim_matches('\0');
    assert_eq!(read_str, "Weapons");

    assert!(m2_py_shop_select_tab(1));
    assert_eq!(m2_py_shop_get_selected_tab(), 1);

    assert!(!m2_py_shop_select_tab(10)); // Out of bounds
}
