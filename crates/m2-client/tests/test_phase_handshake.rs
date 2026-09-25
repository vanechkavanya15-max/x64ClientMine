use m2_client::phase_handshake::PhaseHandshake;

#[test]
fn test_integration_phase_handshake() {
    let instance = PhaseHandshake::new();
    assert!(instance.is_ready());
}
