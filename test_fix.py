import re

with open('crates/m2-client/tests/test_py_bridge_texttail_full.rs', 'r') as f:
    content = f.read()

# Replace all #[test] with #[test] inside a single test function or serialize them.
# The reviewer said: "group the test scenarios sequentially within a single #[test] function to avoid cargo test parallel execution race conditions."

# Let's completely rewrite the test file to just have one single test function
new_content = """use std::ffi::CString;
#[path = "../src/py_bridge_texttail_full.rs"]
mod py_bridge_texttail_full;
use py_bridge_texttail_full::*;

fn clear_state() {
    get_text_tail_state().clear();
}

#[test]
fn test_all_scenarios_sequentially() {
    // 1. test_register_character_and_show
    clear_state();
    m2_py_texttail_register_character(100);
    {
        let state = get_text_tail_state();
        assert!(state.m_CharacterTextTailMap.contains_key(&100));
        assert!(!state.m_CharacterTextTailList.contains(&100));
    }
    m2_py_texttail_show_character_text_tail(100);
    {
        let state = get_text_tail_state();
        assert!(state.m_CharacterTextTailList.contains(&100));
    }
    m2_py_texttail_hide_character_text_tail(100);
    {
        let state = get_text_tail_state();
        assert!(!state.m_CharacterTextTailList.contains(&100));
    }

    // 2. test_register_item_and_show
    clear_state();
    let text = CString::new("Sword").unwrap();
    m2_py_texttail_register_item(200, text.as_ptr());
    {
        let state = get_text_tail_state();
        assert!(state.m_ItemTextTailMap.contains_key(&200));
        assert_eq!(state.m_ItemTextTailMap.get(&200).unwrap().stText, "Sword");
    }

    // 3. test_hide_all_text_tails
    clear_state();
    m2_py_texttail_register_character(100);
    m2_py_texttail_show_character_text_tail(100);
    let text2 = CString::new("Shield").unwrap();
    m2_py_texttail_register_item(200, text2.as_ptr());
    {
        let mut state = get_text_tail_state();
        state.m_ItemTextTailList.push(200);
        assert_eq!(state.m_CharacterTextTailList.len(), 1);
        assert_eq!(state.m_ItemTextTailList.len(), 1);
    }
    m2_py_texttail_hide_all_text_tail();
    {
        let state = get_text_tail_state();
        assert!(state.m_CharacterTextTailList.is_empty());
        assert!(state.m_ItemTextTailList.is_empty());
    }

    // 4. test_attach_title
    clear_state();
    m2_py_texttail_register_character(300);
    let title = CString::new("King").unwrap();
    m2_py_texttail_attach_title(300, title.as_ptr(), 1.0, 0.0, 0.0);
    {
        let state = get_text_tail_state();
        let tail = state.m_CharacterTextTailMap.get(&300).unwrap();
        assert_eq!(tail.stTitle, Some("King".to_string()));
        assert_eq!(tail.title_color, Some(D3DXCOLOR::new(1.0, 0.0, 0.0, 1.0)));
    }

    // 5. test_show_all_text_tail
    clear_state();
    m2_py_texttail_register_character(100);
    let text3 = CString::new("Shield").unwrap();
    m2_py_texttail_register_item(200, text3.as_ptr());
    m2_py_texttail_show_all_text_tail();
    {
        let state = get_text_tail_state();
        assert!(state.m_CharacterTextTailList.contains(&100));
        assert!(state.m_ItemTextTailList.contains(&200));
    }
}
"""

with open('crates/m2-client/tests/test_py_bridge_texttail_full.rs', 'w') as f:
    f.write(new_content)

print("Test file updated.")
