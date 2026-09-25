use m2_client::phase_select::PhaseSelect;

#[test]
fn test_integration_phase_select() {
    let instance = PhaseSelect::new();
    assert!(instance.is_ready());
}
