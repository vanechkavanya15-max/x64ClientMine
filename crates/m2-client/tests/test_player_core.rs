use m2_client::player_core::PlayerCore;

#[test]
fn test_integration_player_core() {
    let instance = PlayerCore::new();
    assert!(instance.is_ready());
}
