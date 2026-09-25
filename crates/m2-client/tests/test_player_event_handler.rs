use m2_client::player_event_handler::PlayerEventHandler;

#[test]
fn test_integration_player_event_handler() {
    let instance = PlayerEventHandler::new();
    assert!(instance.is_ready());
}
