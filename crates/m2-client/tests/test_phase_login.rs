use m2_client::phase_login::PhaseLogin;

#[test]
fn test_integration_phase_login() {
    let instance = PhaseLogin::new();
    assert!(instance.is_ready());
}
