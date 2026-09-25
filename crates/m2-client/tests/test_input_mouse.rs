use m2_client::input_mouse::InputMouse;

#[test]
fn test_integration_input_mouse() {
    let instance = InputMouse::new();
    assert!(instance.is_ready());
}
