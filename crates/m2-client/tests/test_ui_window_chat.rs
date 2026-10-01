use m2_client::chat_buffer_v2::{
    CHAT_TYPE_COMMAND, CHAT_TYPE_GUILD, CHAT_TYPE_INFO, CHAT_TYPE_NOTICE, CHAT_TYPE_PARTY,
    CHAT_TYPE_SHOUT, CHAT_TYPE_TALKING, CHAT_TYPE_WHISPER, CHAT_TYPE_BIG_NOTICE,
};
use m2_client::ui_window_chat::{get_chat_color, ChatTab, UiChatWindow};

#[test]
fn test_chat_window_initialization() {
    let mut window = UiChatWindow::new(800, 600);
    assert_eq!(window.visible_lines, 5);
    assert_eq!(window.active_tab, ChatTab::All);
    assert_eq!(window.is_edit_mode, false);
    assert_eq!(window.x, 0);
    // Height is 5 * 15 = 75. y is 600 - 75 = 525. Edit mode is false, so edit bar height is 0.
    assert_eq!(window.height, 75);
    assert_eq!(window.y, 525);
}

#[test]
fn test_chat_window_layout() {
    let mut window = UiChatWindow::new(800, 600);
    window.set_visible_lines(10);
    assert_eq!(window.visible_lines, 10);
    // Height is 10 * 15 = 150. Edit mode is false, so +0 = 150. y is 600 - 150 = 450.
    assert_eq!(window.height, 150);
    assert_eq!(window.y, 450);

    // Enter edit mode
    window.handle_key_down(0x0D); // VK_RETURN
    assert_eq!(window.is_edit_mode, true);
    // Height becomes 150 + 20 = 170. y is 600 - 170 = 430.
    assert_eq!(window.height, 170);
    assert_eq!(window.y, 430);

    // Change screen size
    window.set_screen_size(1024, 768);
    assert_eq!(window.height, 170);
    assert_eq!(window.y, 768 - 170);
}

#[test]
fn test_chat_colors() {
    assert_eq!(get_chat_color(CHAT_TYPE_TALKING), [1.0, 1.0, 1.0, 1.0]);
    assert_eq!(get_chat_color(CHAT_TYPE_INFO), [1.0, 0.785, 0.785, 1.0]);
    assert_eq!(get_chat_color(CHAT_TYPE_NOTICE), [1.0, 0.902, 0.730, 1.0]);
    assert_eq!(get_chat_color(CHAT_TYPE_PARTY), [0.542, 1.0, 0.949, 1.0]);
    assert_eq!(get_chat_color(CHAT_TYPE_GUILD), [0.906, 0.847, 1.0, 1.0]);
    assert_eq!(get_chat_color(CHAT_TYPE_COMMAND), [0.658, 1.0, 0.835, 1.0]);
    assert_eq!(get_chat_color(CHAT_TYPE_SHOUT), [0.658, 1.0, 0.835, 1.0]);
    assert_eq!(get_chat_color(CHAT_TYPE_WHISPER), [74.0 / 255.0, 225.0 / 255.0, 74.0 / 255.0, 1.0]);
    assert_eq!(get_chat_color(CHAT_TYPE_BIG_NOTICE), [1.0, 0.902, 0.730, 1.0]);
}

#[test]
fn test_focus_and_input() {
    let mut window = UiChatWindow::new(800, 600);
    
    // Pressing 'a' when not in edit mode shouldn't append
    window.append_input_char('a');
    assert_eq!(window.input_text, "");

    // Press Enter to focus
    window.handle_key_down(0x0D);
    assert_eq!(window.is_edit_mode, true);

    // Type text
    window.append_input_char('h');
    window.append_input_char('i');
    assert_eq!(window.input_text, "hi");

    // Press backspace
    window.handle_key_down(0x08);
    assert_eq!(window.input_text, "h");

    // Press escape to clear and unfocus
    window.handle_key_down(0x1B);
    assert_eq!(window.is_edit_mode, false);
    assert_eq!(window.input_text, "");
}

#[test]
fn test_tab_filtering() {
    let mut window = UiChatWindow::new(800, 600);

    // Default All
    assert_eq!(window.is_chat_visible(CHAT_TYPE_TALKING), true);
    assert_eq!(window.is_chat_visible(CHAT_TYPE_PARTY), true);
    assert_eq!(window.is_chat_visible(CHAT_TYPE_GUILD), true);
    assert_eq!(window.is_chat_visible(CHAT_TYPE_INFO), true);

    // Normal
    window.set_tab(ChatTab::Normal);
    assert_eq!(window.is_chat_visible(CHAT_TYPE_TALKING), true);
    assert_eq!(window.is_chat_visible(CHAT_TYPE_SHOUT), true);
    assert_eq!(window.is_chat_visible(CHAT_TYPE_INFO), true);
    assert_eq!(window.is_chat_visible(CHAT_TYPE_PARTY), false);
    assert_eq!(window.is_chat_visible(CHAT_TYPE_GUILD), false);

    // Party
    window.set_tab(ChatTab::Party);
    assert_eq!(window.is_chat_visible(CHAT_TYPE_PARTY), true);
    assert_eq!(window.is_chat_visible(CHAT_TYPE_TALKING), false);

    // Guild
    window.set_tab(ChatTab::Guild);
    assert_eq!(window.is_chat_visible(CHAT_TYPE_GUILD), true);
    assert_eq!(window.is_chat_visible(CHAT_TYPE_PARTY), false);
}

#[test]
fn test_send_chat_packets() {
    let mut window = UiChatWindow::new(800, 600);
    window.handle_key_down(0x0D); // focus
    
    // Setup message
    window.append_input_char('!'); // Shout prefix
    window.append_input_char('h');
    window.append_input_char('e');
    window.append_input_char('l');
    window.append_input_char('l');
    window.append_input_char('o');

    // Press Enter to send (will call m2_send_chat with SHOUT internally)
    let handled = window.handle_key_down(0x0D);
    assert_eq!(handled, true);
    assert_eq!(window.is_edit_mode, false);
    assert_eq!(window.input_text, "");
}

#[test]
fn test_append_chat_lines() {
    let mut window = UiChatWindow::new(800, 600);
    window.append_chat_line(CHAT_TYPE_TALKING, "Hello World!".to_string(), 1000);
    window.append_chat_line(CHAT_TYPE_PARTY, "Group up!".to_string(), 2000);
    window.append_chat_line(CHAT_TYPE_GUILD, "Guild raid!".to_string(), 3000);

    assert_eq!(window.lines.len(), 3);
    assert_eq!(window.lines[0].text, "Hello World!");

    // Verify retrieval based on active tab
    window.set_tab(ChatTab::All);
    assert_eq!(window.get_visible_chat_lines().len(), 3);

    window.set_tab(ChatTab::Party);
    let visible = window.get_visible_chat_lines();
    assert_eq!(visible.len(), 1);
    assert_eq!(visible[0].text, "Group up!");

    // Exceed visible lines limit
    window.set_tab(ChatTab::All);
    window.set_visible_lines(3);
    window.append_chat_line(CHAT_TYPE_INFO, "System msg".to_string(), 4000);
    // Now we have 4 lines, but only 3 should be visible
    let visible_capped = window.get_visible_chat_lines();
    assert_eq!(visible_capped.len(), 3);
    assert_eq!(visible_capped[0].text, "Group up!"); // Hello world is dropped from view
    assert_eq!(visible_capped[2].text, "System msg");
}

#[test]
fn test_whisper_parsing() {
    let mut window = UiChatWindow::new(800, 600);
    window.handle_key_down(0x0D); // focus
    
    let chars: Vec<char> = "/w Bob Hello there".chars().collect();
    for c in chars {
        window.append_input_char(c);
    }

    // Press Enter to send (will call m2_send_whisper internally)
    let handled = window.handle_key_down(0x0D);
    assert_eq!(handled, true);
    assert_eq!(window.is_edit_mode, false);
    assert_eq!(window.input_text, "");
}
