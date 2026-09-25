use m2_client::phase_game_command::PhaseGameCommand;

#[test]
fn test_integration_phase_game_command() {
    let instance = PhaseGameCommand::new();
    assert!(instance.is_ready());
}
