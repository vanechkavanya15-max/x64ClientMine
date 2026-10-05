use m2_client::py_bridge_quest_runtime_complete::{
    m2_py_quest_get_quest_count, m2_py_quest_clear,
    m2_py_quest_make_quest, m2_py_quest_delete_quest,
    m2_py_quest_set_quest_title, m2_py_quest_set_quest_counter_name,
    m2_py_quest_set_quest_counter_value, m2_py_quest_get_quest_data,
    m2_py_quest_get_quest_name, m2_py_quest_get_quest_icon_file_name,
    m2_py_quest_set_quest_icon_file_name, m2_py_quest_get_quest_index,
    m2_py_quest_set_quest_clock_name, m2_py_quest_set_quest_clock_value,
    m2_py_quest_get_quest_last_time
};
use std::ffi::CString;

#[test]
fn test_quest_runtime_add_update_delete() {
    let _ = m2_py_quest_clear();

    assert_eq!(m2_py_quest_get_quest_count(), 0);

    m2_py_quest_make_quest(101);
    assert_eq!(m2_py_quest_get_quest_count(), 1);

    let title_c = CString::new("Find the Blacksmith").unwrap();
    m2_py_quest_set_quest_title(101, title_c.as_ptr());

    let c_name_c = CString::new("Orcs Killed").unwrap();
    m2_py_quest_set_quest_counter_name(101, c_name_c.as_ptr());
    m2_py_quest_set_quest_counter_value(101, 5);

    // Get Data
    let mut title_buf = [0i8; 256];
    let mut counter_name_buf = [0i8; 256];
    let mut counter_val = 0;

    let res = m2_py_quest_get_quest_data(
        0, // array index
        title_buf.as_mut_ptr(),
        title_buf.len(),
        counter_name_buf.as_mut_ptr(),
        counter_name_buf.len(),
        &mut counter_val,
    );

    assert!(res);
    assert_eq!(counter_val, 5);

    let title_str = unsafe { std::ffi::CStr::from_ptr(title_buf.as_ptr()) }.to_str().unwrap();
    assert_eq!(title_str, "Find the Blacksmith");

    let counter_name_str = unsafe { std::ffi::CStr::from_ptr(counter_name_buf.as_ptr()) }.to_str().unwrap();
    assert_eq!(counter_name_str, "Orcs Killed");

    // Delete
    m2_py_quest_delete_quest(101);
    assert_eq!(m2_py_quest_get_quest_count(), 0);
}

#[test]
fn test_quest_runtime_names_and_icons() {
    let _ = m2_py_quest_clear();

    m2_py_quest_make_quest(202);
    
    let title_c = CString::new("Epic Quest").unwrap();
    m2_py_quest_set_quest_title(202, title_c.as_ptr());

    let mut name_buf = [0i8; 256];
    let res = m2_py_quest_get_quest_name(0, name_buf.as_mut_ptr(), name_buf.len());
    assert!(res);
    
    let name_str = unsafe { std::ffi::CStr::from_ptr(name_buf.as_ptr()) }.to_str().unwrap();
    assert_eq!(name_str, "Epic Quest");

    // Setup clear state again
    let _ = m2_py_quest_clear();
}

#[test]
fn test_quest_runtime_advanced_fields() {
    let _ = m2_py_quest_clear();

    m2_py_quest_make_quest(303);
    
    // Icon test
    let icon_c = CString::new("scroll_open.tga").unwrap();
    m2_py_quest_set_quest_icon_file_name(303, icon_c.as_ptr());

    let mut icon_buf = [0i8; 256];
    let res = m2_py_quest_get_quest_icon_file_name(0, icon_buf.as_mut_ptr(), icon_buf.len());
    assert!(res);
    let icon_str = unsafe { std::ffi::CStr::from_ptr(icon_buf.as_ptr()) }.to_str().unwrap();
    assert_eq!(icon_str, "scroll_open.tga");

    // Index test
    assert_eq!(m2_py_quest_get_quest_index(0), 303);

    // Clock test
    let clock_name_c = CString::new("Time Left").unwrap();
    m2_py_quest_set_quest_clock_name(303, clock_name_c.as_ptr());
    m2_py_quest_set_quest_clock_value(303, 100);

    let mut clock_buf = [0i8; 256];
    let mut last_time = 0;
    let res = m2_py_quest_get_quest_last_time(0, clock_buf.as_mut_ptr(), clock_buf.len(), &mut last_time);
    assert!(res);
    
    let clock_str = unsafe { std::ffi::CStr::from_ptr(clock_buf.as_ptr()) }.to_str().unwrap();
    assert_eq!(clock_str, "Time Left");
    // Depending on when it is run, last_time should be around 100
    assert!(last_time >= 99 && last_time <= 100);

    let _ = m2_py_quest_clear();
}

#[test]
fn test_quest_runtime_clear() {
    let _ = m2_py_quest_clear();
    
    m2_py_quest_make_quest(1);
    m2_py_quest_make_quest(2);
    m2_py_quest_make_quest(3);

    assert_eq!(m2_py_quest_get_quest_count(), 3);

    m2_py_quest_clear();

    assert_eq!(m2_py_quest_get_quest_count(), 0);
}

#[test]
fn test_quest_runtime_update_existing_quest() {
    let _ = m2_py_quest_clear();

    m2_py_quest_make_quest(10);
    let title1 = CString::new("Old Title").unwrap();
    m2_py_quest_set_quest_title(10, title1.as_ptr());

    let mut name_buf = [0i8; 256];
    m2_py_quest_get_quest_name(0, name_buf.as_mut_ptr(), name_buf.len());
    let name_str1 = unsafe { std::ffi::CStr::from_ptr(name_buf.as_ptr()) }.to_str().unwrap();
    assert_eq!(name_str1, "Old Title");

    let title2 = CString::new("New Title").unwrap();
    m2_py_quest_set_quest_title(10, title2.as_ptr());

    m2_py_quest_get_quest_name(0, name_buf.as_mut_ptr(), name_buf.len());
    let name_str2 = unsafe { std::ffi::CStr::from_ptr(name_buf.as_ptr()) }.to_str().unwrap();
    assert_eq!(name_str2, "New Title");

    let _ = m2_py_quest_clear();
}
