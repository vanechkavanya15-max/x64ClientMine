use m2_client_pyapi::mod_chat::ModChat;

#[test]
fn test_integration_mod_chat() {
    let instance = ModChat::new();
    assert!(instance.is_ready());
}
