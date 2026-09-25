use m2_client::phase_game_actor::PhaseGameActor;

#[test]
fn test_integration_phase_game_actor() {
    let instance = PhaseGameActor::new();
    assert!(instance.is_ready());
}
