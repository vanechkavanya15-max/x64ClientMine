use m2_client::phase_loading::PhaseLoading;

#[test]
fn test_integration_phase_loading() {
    let instance = PhaseLoading::new();
    assert!(instance.is_ready());
}
