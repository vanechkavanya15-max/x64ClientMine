use m2_client_pyapi::mod_exchange::ModExchange;

#[test]
fn test_integration_mod_exchange() {
    let instance = ModExchange::new();
    assert!(instance.is_ready());
}
