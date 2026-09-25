use m2_client::phase_game_main::PhaseGameMain;

#[test]
fn test_integration_phase_game_main() {
    let instance = PhaseGameMain::new();
    assert!(instance.is_ready());
}
