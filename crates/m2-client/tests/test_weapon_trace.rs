use m2_client::weapon_trace::WeaponTrace;

#[test]
fn test_integration_weapon_trace() {
    let instance = WeaponTrace::new();
    assert!(instance.is_ready());
}
