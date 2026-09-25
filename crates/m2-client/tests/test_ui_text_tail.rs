use m2_client::ui_text_tail::UiTextTail;

#[test]
fn test_integration_ui_text_tail() {
    let instance = UiTextTail::new();
    assert!(instance.is_ready());
}
