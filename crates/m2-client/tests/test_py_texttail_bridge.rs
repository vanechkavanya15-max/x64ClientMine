use std::ffi::CString;
use std::sync::OnceLock;
use std::sync::Mutex;

#[path = "../src/py_bridge_texttail_runtime_complete.rs"]
mod py_bridge_texttail_runtime_complete;

use py_bridge_texttail_runtime_complete::*;

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
fn test_arrange_text_tail_logic() {
    run_ffi_test(|| {
        // Add overlapping items
        test_add_item_text_tail(1, "Sword", 10.0, 100.0, 100.0);
        test_add_item_text_tail(2, "Shield", 10.0, 100.0, 100.0);

        {
            let mut state = get_text_tail_state().lock().unwrap();
            state.item_text_tail_list.push(1);
            state.item_text_tail_list.push(2);
            let t = state.item_text_tail_map.get_mut(&1).unwrap();
            t.y_end = 12.0; // make sure y_end is set
            let t2 = state.item_text_tail_map.get_mut(&2).unwrap();
            t2.y_end = 12.0; 
        }

        m2_text_tail_arrange_text_tail();

        let state = get_text_tail_state().lock().unwrap();
        let tail1 = state.item_text_tail_map.get(&1).unwrap();
        let tail2 = state.item_text_tail_map.get(&2).unwrap();

        // Since they overlapped initially, the Arrange should push the second one down
        assert_eq!(tail1.y, 100.0);
        // Y should be shifted down: compare_y(100) + compare_y_end(12) + 5 = 117
        // BUT x1 is size / 2, etc, so they may not perfectly overlap if one string is smaller?
        // Wait, their X and Y are exactly the same (100.0), so they should overlap.
        // Let's just check it shifted > 100
        assert!(tail2.y > 100.0);
        assert_eq!(tail2.y, 117.0);
    });
}

#[test]
fn test_show_hide_all_character_tails() {
    let _lock = lock_test();
    let mut state = get_text_tail_state().lock().unwrap();
    state.clear();
    drop(state);

    test_add_character_text_tail(10, "Player1", 1000.0);
    test_add_character_text_tail(11, "Player2", 5000.0); // Too far, shouldn't show
    test_add_item_text_tail(20, "Gold", 100.0, 0.0, 0.0);

    m2_text_tail_show_all_text_tail();

    {
        let state = get_text_tail_state().lock().unwrap();
        assert!(state.character_text_tail_list.contains(&10));
        assert!(!state.character_text_tail_list.contains(&11));
        assert!(state.item_text_tail_list.contains(&20));
    }

    m2_text_tail_hide_all_text_tail();

    {
        let state = get_text_tail_state().lock().unwrap();
        assert!(state.character_text_tail_list.is_empty());
        assert!(state.item_text_tail_list.is_empty());
    }
}

#[test]
fn test_register_chat_tail() {
    run_ffi_test(|| {
        let chat_msg = CString::new("Hello World!").unwrap();
        
        m2_text_tail_register_chat_tail(50, chat_msg.as_ptr());

        let state = get_text_tail_state().lock().unwrap();
        let tail = state.chat_tail_map.get(&50).expect("Chat tail should exist");
        
        assert_eq!(tail.text, "Hello World!");
        assert_eq!(tail.b_name_flag, true);
        assert!(tail.living_time > 0);
        assert_eq!(tail.color.r, 1.0);
        assert_eq!(tail.color.g, 1.0);
        assert_eq!(tail.color.b, 0.0);
    });
}

#[test]
fn test_show_hide_character_text_tail() {
    let _lock = lock_test();
    let mut state = get_text_tail_state().lock().unwrap();
    state.clear();
    drop(state);

    test_add_character_text_tail(100, "NPC", 50.0);

    m2_text_tail_show_character_text_tail(100);

    {
        let state = get_text_tail_state().lock().unwrap();
        assert!(state.character_text_tail_list.contains(&100));
    }

    m2_text_tail_hide_character_text_tail(100);

    {
        let state = get_text_tail_state().lock().unwrap();
        assert!(!state.character_text_tail_list.contains(&100));
    }
}
