use m2_client::py_bridge_quest::*;
use m2_client::ui_quest_dialog::get_ui_quest_dialog;
use std::ffi::CString;

static TEST_LOCK: std::sync::OnceLock<std::sync::Mutex<()>> = std::sync::OnceLock::new();

fn lock_test() -> std::sync::MutexGuard<'static, ()> {
    TEST_LOCK.get_or_init(|| std::sync::Mutex::new(())).lock().unwrap()
}

#[test]
fn test_quest_clear() {
    let _guard = lock_test();
    m2_py_quest_set_skin(2);
    m2_py_quest_set_clock_value(100);
    
    m2_py_quest_clear();
    
    let state = get_ui_quest_dialog().lock().unwrap();
    assert_eq!(state.skin_type, 0);
    assert_eq!(state.clock_value, 0);
}

#[test]
fn test_quest_set_skin() {
    let _guard = lock_test();
    m2_py_quest_clear();
    m2_py_quest_set_skin(5);
    
    let state = get_ui_quest_dialog().lock().unwrap();
    assert_eq!(state.skin_type, 5);
}

#[test]
fn test_quest_set_title() {
    let _guard = lock_test();
    m2_py_quest_clear();
    let title = CString::new("Main Quest").unwrap();
    m2_py_quest_set_title(title.as_ptr());
    
    let state = get_ui_quest_dialog().lock().unwrap();
    assert_eq!(state.title, "Main Quest");
}

#[test]
fn test_quest_set_clock_title() {
    let _guard = lock_test();
    m2_py_quest_clear();
    let title = CString::new("Time Left").unwrap();
    m2_py_quest_set_clock_title(title.as_ptr());
    
    let state = get_ui_quest_dialog().lock().unwrap();
    assert_eq!(state.clock_title, "Time Left");
}

#[test]
fn test_quest_set_clock_value() {
    let _guard = lock_test();
    m2_py_quest_clear();
    m2_py_quest_set_clock_value(120);
    
    let state = get_ui_quest_dialog().lock().unwrap();
    assert_eq!(state.clock_value, 120);
}

#[test]
fn test_quest_set_counter_name() {
    let _guard = lock_test();
    m2_py_quest_clear();
    let name = CString::new("Monsters Killed").unwrap();
    m2_py_quest_set_counter_name(name.as_ptr());
    
    let state = get_ui_quest_dialog().lock().unwrap();
    assert_eq!(state.counter_name, "Monsters Killed");
}

#[test]
fn test_quest_set_counter_value() {
    let _guard = lock_test();
    m2_py_quest_clear();
    m2_py_quest_set_counter_value(50);
    
    let state = get_ui_quest_dialog().lock().unwrap();
    assert_eq!(state.counter_value, 50);
}
