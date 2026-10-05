use std::sync::{Mutex, OnceLock};

#[path = "../src/py_bridge_texttail_runtime_complete.rs"]
mod py_bridge_texttail_runtime_complete;

#[path = "../src/py_bridge_texttail_query.rs"]
mod py_bridge_texttail_query;

use py_bridge_texttail_runtime_complete::*;
use py_bridge_texttail_query::*;

static TEST_MUTEX: OnceLock<Mutex<()>> = OnceLock::new();

fn lock_test() -> std::sync::MutexGuard<'static, ()> {
    TEST_MUTEX.get_or_init(|| Mutex::new(())).lock().unwrap()
}

// Ensure the FFI functions don't panic due to missing CatchUnwind bounds in tests
fn run_ffi_test<F: FnOnce()>(f: F) {
    let _lock = lock_test();
    let mut state = get_text_tail_state().lock().unwrap();
    state.clear();
    drop(state); // Drop the lock before running FFI!
    f();
}

#[test]
fn test_get_position() {
    run_ffi_test(|| {
        test_add_character_text_tail(1, "Player", 10.0);
        
        {
            let mut state = get_text_tail_state().lock().unwrap();
            if let Some(t) = state.character_text_tail_map.get_mut(&1) {
                t.x = 100.0;
                t.y = 200.0;
                t.z = 300.0;
            }
        }

        let mut x: f32 = 0.0;
        let mut y: f32 = 0.0;
        let mut z: f32 = 0.0;

        // Test existing
        let exists = m2_text_tail_get_position(1, &mut x, &mut y, &mut z);
        assert!(exists);
        assert_eq!(x, 100.0);
        assert_eq!(y, 200.0);
        assert_eq!(z, 300.0);

        // Test non-existing
        let exists_none = m2_text_tail_get_position(2, &mut x, &mut y, &mut z);
        assert!(!exists_none);
    });
}

#[test]
fn test_show_item_text_tail() {
    run_ffi_test(|| {
        test_add_item_text_tail(10, "Sword", 5.0, 0.0, 0.0);

        m2_text_tail_show_item_text_tail(10);

        {
            let state = get_text_tail_state().lock().unwrap();
            assert!(state.item_text_tail_list.contains(&10));
        }

        // Test double show (should not duplicate)
        m2_text_tail_show_item_text_tail(10);
        {
            let state = get_text_tail_state().lock().unwrap();
            assert_eq!(state.item_text_tail_list.len(), 1);
        }
    });
}

#[test]
fn test_select_item_name() {
    run_ffi_test(|| {
        test_add_item_text_tail(20, "Shield", 5.0, 0.0, 0.0);

        m2_text_tail_select_item_name(20);

        {
            let state = get_text_tail_state().lock().unwrap();
            let tail = state.item_text_tail_map.get(&20).unwrap();
            assert_eq!(tail.color.r, 0.1);
            assert_eq!(tail.color.g, 0.9);
            assert_eq!(tail.color.b, 0.1);
        }
    });
}

#[test]
fn test_is_chat() {
    run_ffi_test(|| {
        use std::ffi::CString;

        let chat_msg = CString::new("Hello").unwrap();
        m2_text_tail_register_chat_tail(30, chat_msg.as_ptr());

        assert!(m2_text_tail_is_chat(30));
        assert!(!m2_text_tail_is_chat(31));
    });
}

#[test]
fn test_clear() {
    run_ffi_test(|| {
        test_add_character_text_tail(1, "Player", 10.0);
        test_add_item_text_tail(10, "Sword", 5.0, 0.0, 0.0);
        
        {
            let mut state = get_text_tail_state().lock().unwrap();
            state.character_text_tail_list.push(1);
            state.item_text_tail_list.push(10);
        }

        m2_text_tail_clear();

        let state = get_text_tail_state().lock().unwrap();
        assert!(state.character_text_tail_map.is_empty());
        assert!(state.item_text_tail_map.is_empty());
        assert!(state.character_text_tail_list.is_empty());
        assert!(state.item_text_tail_list.is_empty());
        assert!(state.chat_tail_map.is_empty());
    });
}
