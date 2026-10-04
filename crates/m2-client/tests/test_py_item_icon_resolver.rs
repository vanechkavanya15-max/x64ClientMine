use m2_client::py_bridge_item_icon_resolver::*;
use std::ffi::CString;

#[test]
fn test_default_item_icon_resolution() {
    let mut buf = [0u8; 64];
    assert!(m2_py_item_get_icon_image_path(10, buf.as_mut_ptr() as *mut _, buf.len()));
    let path = std::str::from_utf8(&buf).unwrap().trim_matches('\0');
    assert_eq!(path, "icon/item/00010.tga");

    let mut w = 0u8;
    let mut h = 0u8;
    assert!(m2_py_item_get_size_in_slots(10, &mut w, &mut h));
    assert_eq!(w, 1);
    assert_eq!(h, 2); // Weapon is 1x2 slots
}

#[test]
fn test_item_icon_override() {
    let custom_path = CString::new("custom/path/sword.tga").unwrap();
    assert!(m2_py_item_register_icon_override(99999, custom_path.as_ptr(), 1, 3));

    let mut buf = [0u8; 64];
    assert!(m2_py_item_get_icon_image_path(99999, buf.as_mut_ptr() as *mut _, buf.len()));
    let path = std::str::from_utf8(&buf).unwrap().trim_matches('\0');
    assert_eq!(path, "custom/path/sword.tga");

    let mut w = 0u8;
    let mut h = 0u8;
    assert!(m2_py_item_get_size_in_slots(99999, &mut w, &mut h));
    assert_eq!(w, 1);
    assert_eq!(h, 3);
}
