use m2_client_pyapi::mod_net::ModNet;

#[test]
fn test_integration_mod_net() {
    let instance = ModNet::new();
    assert!(instance.is_ready());
}
