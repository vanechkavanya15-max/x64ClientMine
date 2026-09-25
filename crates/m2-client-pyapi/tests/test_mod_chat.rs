use m2_client_pyapi::mod_chat::{ChatMessage, ModChat};

#[test]
fn test_integration_mod_chat() {
    let instance = ModChat::new();
    assert!(instance.is_ready());
}

#[test]
fn test_integration_append_chat() {
    let mut chat = ModChat::new();
    chat.append_chat(1, "Czesc".to_string());
    assert_eq!(chat.get_line_count(), 1);
    assert_eq!(
        chat.messages[0],
        ChatMessage {
            chat_type: 1,
            message: "Czesc".to_string(),
            delay: 0,
        }
    );
}

#[test]
fn test_integration_append_chat_with_delay() {
    let mut chat = ModChat::new();
    chat.append_chat_with_delay(3, "Wiadomosc z opoznieniem".to_string(), 100);
    assert_eq!(chat.get_line_count(), 1);
    assert_eq!(
        chat.messages[0],
        ChatMessage {
            chat_type: 3,
            message: "Wiadomosc z opoznieniem".to_string(),
            delay: 100,
        }
    );
}

#[test]
fn test_integration_chat_filters() {
    let mut chat = ModChat::new();
    chat.set_filter(5);
    chat.append_chat(5, "Zignorowana".to_string());
    assert_eq!(chat.get_line_count(), 0);

    chat.append_chat_with_delay(5, "Rowniez zignorowana".to_string(), 50);
    assert_eq!(chat.get_line_count(), 0);

    chat.remove_filter(5);
    chat.append_chat(5, "Zauwazona".to_string());
    assert_eq!(chat.get_line_count(), 1);
}
