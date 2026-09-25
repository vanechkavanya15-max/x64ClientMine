use m2_client::ui_text_tail::{UiTextTail, Color};

#[test]
fn test_integration_ui_text_tail() {
    let mut instance = UiTextTail::new();
    assert!(instance.is_ready());

    // Register elements
    instance.register_character_text_tail(1, 100, Color::new(1.0, 1.0, 1.0, 1.0), 10.0, "Player1", "Guild1");
    assert!(instance.character_text_tail_map.contains_key(&100));

    instance.register_item_text_tail(200, "Sword");
    assert!(instance.item_text_tail_map.contains_key(&200));

    instance.register_chat_tail(100, "Hello", 1000);
    assert!(instance.chat_tail_map.contains_key(&100));

    // Visibility Management
    instance.show_character_text_tail(100);
    assert!(instance.character_text_tail_list.contains(&100));
    
    instance.show_item_text_tail(200);
    assert!(instance.item_text_tail_list.contains(&200));

    // Overlapping logic implicitly tested if it doesn't crash
    instance.arrange_text_tail(1500);

    // Color updates
    instance.set_character_text_tail_color(100, Color::new(1.0, 0.0, 0.0, 1.0));
    let color = instance.character_text_tail_map.get(&100).unwrap().color.r;
    assert_eq!(color, 1.0);

    // Deletion
    instance.delete_character_text_tail(100);
    assert!(!instance.character_text_tail_map.contains_key(&100));
    assert!(!instance.character_text_tail_list.contains(&100));

    instance.delete_item_text_tail(200);
    assert!(!instance.item_text_tail_map.contains_key(&200));
    assert!(!instance.item_text_tail_list.contains(&200));
}
