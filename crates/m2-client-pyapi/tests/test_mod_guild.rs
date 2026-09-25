use m2_client_pyapi::mod_guild::ModGuild;

#[test]
fn test_integration_mod_guild() {
    let instance = ModGuild::new();
    assert!(instance.is_ready());
}
