use m2_client::player_input_logic::PlayerInputLogic;

#[test]
fn test_integration_player_input_logic() {
    let instance = PlayerInputLogic::new();
    assert!(instance.is_ready());
}
