use m2_client_pyapi::mod_safebox::ModSafebox;

#[test]
fn test_integration_mod_safebox() {
    let instance = ModSafebox::new();
    assert!(instance.is_ready());
}
