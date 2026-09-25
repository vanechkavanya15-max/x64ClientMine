use m2_client::phase_game_item::PhaseGameItem;

#[test]
fn test_integration_phase_game_item() {
    let instance = PhaseGameItem::new();
    assert!(instance.is_ready());
}
