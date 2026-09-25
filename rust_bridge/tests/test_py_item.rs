use m2_client_bridge::api::py_item::{
    m2_py_item_get_type, m2_py_item_get_price, m2_py_item_get_name,
    ITEM_NONE, ITEM_WEAPON, ITEM_ARMOR, ITEM_USE,
};
use std::os::raw::c_char;
use std::ffi::CStr;

#[test]
fn test_py_item_get_type() {
    unsafe {
        assert_eq!(m2_py_item_get_type(10), ITEM_WEAPON); // Miecz +0
        assert_eq!(m2_py_item_get_type(11200), ITEM_ARMOR); // Mnisia Zbroja Plytowa +0
        assert_eq!(m2_py_item_get_type(27000), ITEM_USE); // Czerwona Mikstura
        assert_eq!(m2_py_item_get_type(99999), ITEM_NONE); // Nieznany vnum
    }
}

#[test]
fn test_py_item_get_price() {
    unsafe {
        assert_eq!(m2_py_item_get_price(10), 1000); // Miecz +0
        assert_eq!(m2_py_item_get_price(11200), 5000); // Mnisia Zbroja Plytowa +0
        assert_eq!(m2_py_item_get_price(27000), 50); // Czerwona Mikstura
        assert_eq!(m2_py_item_get_price(99999), 0); // Nieznany vnum
    }
}

#[test]
fn test_py_item_get_name_success() {
    let mut buf = [0u8; 64];
    unsafe {
        let result = m2_py_item_get_name(10, buf.as_mut_ptr() as *mut c_char, buf.len());
        assert!(result);
        let name = CStr::from_ptr(buf.as_ptr() as *const c_char).to_str().unwrap();
        assert_eq!(name, "Miecz +0");
        
        let result = m2_py_item_get_name(11200, buf.as_mut_ptr() as *mut c_char, buf.len());
        assert!(result);
        let name = CStr::from_ptr(buf.as_ptr() as *const c_char).to_str().unwrap();
        assert_eq!(name, "Mnisia Zbroja Plytowa +0");
    }
}

#[test]
fn test_py_item_get_name_buffer_too_small() {
    let mut buf = [0u8; 6];
    unsafe {
        let result = m2_py_item_get_name(10, buf.as_mut_ptr() as *mut c_char, buf.len());
        assert!(result);
        // Max len is 6, so copy_len is min("Miecz +0".len(), 5) = 5
        let name = CStr::from_ptr(buf.as_ptr() as *const c_char).to_str().unwrap();
        assert_eq!(name, "Miecz");
        assert_eq!(buf[5], 0); // null terminator is at buf[max_len - 1]
    }
}

#[test]
fn test_py_item_get_name_invalid_args() {
    unsafe {
        let result = m2_py_item_get_name(10, std::ptr::null_mut(), 64);
        assert!(!result);

        let mut buf = [0u8; 64];
        let result = m2_py_item_get_name(10, buf.as_mut_ptr() as *mut c_char, 0);
        assert!(!result);
    }
}
