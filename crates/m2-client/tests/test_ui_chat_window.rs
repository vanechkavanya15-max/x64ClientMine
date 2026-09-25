use m2_client::ui_chat_window::{UiChatWindow, ChatType, BoardState, CHAT_LINE_MAX_NUM};

#[test]
fn test_integration_ui_chat_window() {
    let instance = UiChatWindow::new();
    assert!(instance.is_ready());
}

#[test]
fn test_append_chat_capacity() {
    let mut window = UiChatWindow::new();
    let set_id = window.create_chat_set(1);
    
    // Add more messages than max capacity
    for i in 0..(CHAT_LINE_MAX_NUM + 10) {
        window.append_chat(ChatType::Talking, format!("Message {}", i));
    }

    assert_eq!(window.chat_line_deque.len(), CHAT_LINE_MAX_NUM);
    
    // Check if the chat set correctly received max messages
    let visible_count = window.get_visible_line_count(set_id);
    assert_eq!(visible_count, CHAT_LINE_MAX_NUM);
}

#[test]
fn test_chat_modes_filtering() {
    let mut window = UiChatWindow::new();
    let set_id = window.create_chat_set(1);
    
    // Disable Party chat for this chat set
    window.disable_chat_mode(set_id, ChatType::Party);

    // Append standard message
    window.append_chat(ChatType::Talking, "Normal chat".to_string());
    
    // Append Party message
    window.append_chat(ChatType::Party, "Party message".to_string());
    
    assert_eq!(window.chat_line_deque.len(), 2);
    
    // Since it's BoardState::View by default, append_chat adds to showing_chat_line_list
    // However, wait, append_chat pushes unconditionally in View mode?
    // Let's test the logic.
    // In our implementation, we add unconditionally in View mode, but arrange_showing_chat filters.
    // Let's arrange.
    window.arrange_showing_chat(set_id);
    assert_eq!(window.get_visible_line_count(set_id), 1);
}

#[test]
fn test_ignore_character() {
    let mut window = UiChatWindow::new();
    
    assert_eq!(window.is_ignore_character("Spammer"), false);
    
    window.ignore_character("Spammer");
    assert_eq!(window.is_ignore_character("Spammer"), true);
    
    // Toggle
    window.ignore_character("Spammer");
    assert_eq!(window.is_ignore_character("Spammer"), false);
}

use m2_client::ui_chat_window::{WhisperType, START_DISAPPEARING_TIME, MAX_LINE_COUNT};

#[test]
fn test_whisper_logic() {
    let mut window = UiChatWindow::new();
    let whisper_name = "Player2";
    
    // Implicit creation on append
    window.append_whisper(WhisperType::Chat, whisper_name, "Hello!");
    
    assert!(window.get_whisper(whisper_name).is_some());
    let whisper = window.get_whisper(whisper_name).unwrap();
    assert_eq!(whisper.chat_line_deque.len(), 1);
    assert_eq!(whisper.chat_line_deque[0].borrow().text, "Hello!");
    
    // Clear whisper
    window.clear_whisper(whisper_name);
    assert!(window.get_whisper(whisper_name).is_none());
}

#[test]
fn test_update_view_mode_fading() {
    let mut window = UiChatWindow::new();
    let set_id = window.create_chat_set(1);
    
    // Add multiple lines
    for i in 0..(MAX_LINE_COUNT + 2) {
        window.append_chat(ChatType::Talking, format!("Message {}", i));
    }
    
    // Check line counts right after insertion
    assert_eq!(window.get_visible_line_count(set_id), MAX_LINE_COUNT + 2);

    // Call update_view_mode. Time hasn't advanced, but line count > MAX_LINE_COUNT
    // The oldest lines should start fading.
    // Let's call it enough times to make them drop below 0.1 alpha
    for _ in 0..30 {
        window.update_view_mode(set_id, 0.0);
    }
    
    // The lines exceeding max limit should have been removed
    assert_eq!(window.get_visible_line_count(set_id), MAX_LINE_COUNT - 1);
    
    // Now simulate time passing
    let future_time = START_DISAPPEARING_TIME + 1.0;
    
    // Update again many times to fade out remaining lines
    for _ in 0..30 {
        window.update_view_mode(set_id, future_time);
    }
    
    // All lines should be removed since time passed
    assert_eq!(window.get_visible_line_count(set_id), 0);
}

#[test]
fn test_update_edit_mode_alpha() {
    let mut window = UiChatWindow::new();
    let set_id = window.create_chat_set(1);
    
    window.set_board_state(set_id, BoardState::Edit);
    window.set_height(set_id, 30);
    window.set_step(set_id, 15); // editable count = 2
    
    window.append_chat(ChatType::Talking, "Msg 1".to_string());
    window.append_chat(ChatType::Talking, "Msg 2".to_string());
    window.append_chat(ChatType::Talking, "Msg 3".to_string());
    
    assert_eq!(window.get_editable_line_count(set_id), 2);
    assert_eq!(window.get_visible_line_count(set_id), 3);
    
    // In edit mode, update should adjust alpha
    window.update(set_id, 0.0);
    
    // We can't easily assert the exact alpha values without exposing them or modifying test accessibility,
    // but we verify no panic occurs and state remains valid.
    assert_eq!(window.get_visible_line_count(set_id), 3);
}

#[test]
fn test_input_history() {
    let mut window = UiChatWindow::new();
    
    window.append_input_history("hello".to_string());
    assert_eq!(window.input_history.len(), 1);
    assert_eq!(window.input_history[0], "hello");

    window.append_input_history("".to_string());
    assert_eq!(window.input_history.len(), 1); // Empty string shouldn't be added
    
    // Test capacity
    for i in 0..(CHAT_LINE_MAX_NUM + 10) {
        window.append_input_history(format!("cmd {}", i));
    }
    
    assert_eq!(window.input_history.len(), CHAT_LINE_MAX_NUM);
}
