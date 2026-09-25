use m2_client_pyapi::mod_app::ModApp;

#[test]
fn test_integration_mod_app() {
    let instance = ModApp::new();
    assert!(instance.is_ready());
}
