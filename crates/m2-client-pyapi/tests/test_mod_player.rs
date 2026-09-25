use m2_client_pyapi::mod_player::ModPlayer;

#[test]
fn test_integration_mod_player() {
    let instance = ModPlayer::new();
    assert!(instance.is_ready());
}
