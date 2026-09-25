use m2_client::input_keyboard::InputKeyboard;

#[test]
fn test_integration_input_keyboard() {
    let instance = InputKeyboard::new();
    assert!(instance.is_ready());
}
