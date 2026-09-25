use m2_client_pyapi::mod_guild::ModGuild;

#[test]
fn test_integration_mod_guild() {
    let instance = ModGuild::new();
    assert!(instance.is_ready());
}

#[test]
fn test_integration_mod_guild_defaults() {
    let instance = ModGuild::new();
    assert_eq!(instance.get_guild_id(), 0);
    assert_eq!(instance.get_guild_name(), "");
    assert_eq!(instance.get_member_count(), 0);
}

#[test]
fn test_integration_mod_guild_set_data() {
    let mut instance = ModGuild::new();
    instance.set_guild_data(999, "Smoki".to_string(), 120);
    assert_eq!(instance.get_guild_id(), 999);
    assert_eq!(instance.get_guild_name(), "Smoki");
    assert_eq!(instance.get_member_count(), 120);
}
