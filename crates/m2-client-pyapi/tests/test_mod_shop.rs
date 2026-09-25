use m2_client_pyapi::mod_shop::ModShop;

#[test]
fn test_integration_mod_shop() {
    let instance = ModShop::new();
    assert!(instance.is_ready());
}
