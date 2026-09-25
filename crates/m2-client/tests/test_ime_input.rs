use m2_client::ime_input::ImeInput;

#[test]
fn test_integration_ime_input() {
    let instance = ImeInput::new();
    assert!(instance.is_ready());
}
